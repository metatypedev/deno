// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

//! There are many types of errors in Deno:
//! - AnyError: a generic wrapper that can encapsulate any type of error.
//! - JsError: a container for the error message and stack trace for exceptions
//!   thrown in JavaScript code. We use this to pretty-print stack traces.
//! - Diagnostic: these are errors that originate in TypeScript's compiler.
//!   They're similar to JsError, in that they have line numbers. But
//!   Diagnostics are compile-time type errors, whereas JsErrors are runtime
//!   exceptions.

use deno_broadcast_channel::BroadcastChannelError;
use deno_cache::CacheError;
use deno_canvas::CanvasError;
use deno_core::error::AnyError;
use deno_core::serde_json;
use deno_core::url;
use deno_core::ModuleResolutionError;
use deno_cron::CronError;
use deno_ffi::CallError;
use deno_ffi::CallbackError;
use deno_ffi::DlfcnError;
use deno_ffi::IRError;
use deno_ffi::ReprError;
use deno_ffi::StaticError;
use deno_kv::KvCheckError;
use deno_kv::KvError;
use deno_kv::KvMutationError;
use deno_net::ops::NetError;
use deno_tls::TlsError;
use deno_web::BlobError;
use deno_web::CompressionError;
use deno_web::MessagePortError;
use deno_web::StreamResourceError;
use deno_web::WebError;
use deno_webstorage::WebStorageError;
use std::env;
use std::error::Error;
use std::io;
use std::sync::Arc;

fn get_dlopen_error_class(error: &dlopen2::Error) -> &'static str {
  use dlopen2::Error::*;
  match error {
    NullCharacter(_) => "InvalidData",
    OpeningLibraryError(ref e) => get_io_error_class(e),
    SymbolGettingError(ref e) => get_io_error_class(e),
    AddrNotMatchingDll(ref e) => get_io_error_class(e),
    NullSymbol => "NotFound",
  }
}

fn get_env_var_error_class(error: &env::VarError) -> &'static str {
  use env::VarError::*;
  match error {
    NotPresent => "NotFound",
    NotUnicode(..) => "InvalidData",
  }
}

fn get_io_error_class(error: &io::Error) -> &'static str {
  use io::ErrorKind::*;
  match error.kind() {
    NotFound => "NotFound",
    PermissionDenied => "PermissionDenied",
    ConnectionRefused => "ConnectionRefused",
    ConnectionReset => "ConnectionReset",
    ConnectionAborted => "ConnectionAborted",
    NotConnected => "NotConnected",
    AddrInUse => "AddrInUse",
    AddrNotAvailable => "AddrNotAvailable",
    BrokenPipe => "BrokenPipe",
    AlreadyExists => "AlreadyExists",
    InvalidInput => "TypeError",
    InvalidData => "InvalidData",
    TimedOut => "TimedOut",
    Interrupted => "Interrupted",
    WriteZero => "WriteZero",
    UnexpectedEof => "UnexpectedEof",
    Other => "Error",
    WouldBlock => "WouldBlock",
    // Non-exhaustive enum - might add new variants
    // in the future
    kind => {
      let kind_str = kind.to_string();
      match kind_str.as_str() {
        "FilesystemLoop" => "FilesystemLoop",
        "IsADirectory" => "IsADirectory",
        "NetworkUnreachable" => "NetworkUnreachable",
        "NotADirectory" => "NotADirectory",
        _ => "Error",
      }
    }
  }
}

fn get_module_resolution_error_class(
  _: &ModuleResolutionError,
) -> &'static str {
  "URIError"
}

fn get_notify_error_class(error: &notify::Error) -> &'static str {
  use notify::ErrorKind::*;
  match error.kind {
    Generic(_) => "Error",
    Io(ref e) => get_io_error_class(e),
    PathNotFound => "NotFound",
    WatchNotFound => "NotFound",
    InvalidConfig(_) => "InvalidData",
    MaxFilesWatch => "Error",
  }
}

fn get_regex_error_class(error: &regex::Error) -> &'static str {
  use regex::Error::*;
  match error {
    Syntax(_) => "SyntaxError",
    CompiledTooBig(_) => "RangeError",
    _ => "Error",
  }
}

fn get_serde_json_error_class(
  error: &serde_json::error::Error,
) -> &'static str {
  use deno_core::serde_json::error::*;
  match error.classify() {
    Category::Io => error
      .source()
      .and_then(|e| e.downcast_ref::<io::Error>())
      .map(get_io_error_class)
      .unwrap(),
    Category::Syntax => "SyntaxError",
    Category::Data => "InvalidData",
    Category::Eof => "UnexpectedEof",
  }
}

fn get_url_parse_error_class(_error: &url::ParseError) -> &'static str {
  "URIError"
}

fn get_hyper_error_class(_error: &hyper::Error) -> &'static str {
  "Http"
}

fn get_hyper_util_error_class(
  _error: &hyper_util::client::legacy::Error,
) -> &'static str {
  "Http"
}

fn get_hyper_v014_error_class(_error: &hyper_v014::Error) -> &'static str {
  "Http"
}

#[cfg(unix)]
pub fn get_nix_error_class(error: &nix::Error) -> &'static str {
  match error {
    nix::Error::ECHILD => "NotFound",
    nix::Error::EINVAL => "TypeError",
    nix::Error::ENOENT => "NotFound",
    nix::Error::ENOTTY => "BadResource",
    nix::Error::EPERM => "PermissionDenied",
    nix::Error::ESRCH => "NotFound",
    nix::Error::ELOOP => "FilesystemLoop",
    nix::Error::ENOTDIR => "NotADirectory",
    nix::Error::ENETUNREACH => "NetworkUnreachable",
    nix::Error::EISDIR => "IsADirectory",
    nix::Error::UnknownErrno => "Error",
    &nix::Error::ENOTSUP => unreachable!(),
    _ => "Error",
  }
}

fn get_web_error_class(e: &WebError) -> &'static str {
  match e {
    WebError::Base64Decode => "DOMExceptionInvalidCharacterError",
    WebError::InvalidEncodingLabel(_) => "RangeError",
    WebError::BufferTooLong => "TypeError",
    WebError::ValueTooLarge => "RangeError",
    WebError::BufferTooSmall => "RangeError",
    WebError::DataInvalid => "TypeError",
    WebError::DataError(_) => "Error",
  }
}

fn get_web_compression_error_class(e: &CompressionError) -> &'static str {
  match e {
    CompressionError::UnsupportedFormat => "TypeError",
    CompressionError::ResourceClosed => "TypeError",
    CompressionError::IoTypeError(_) => "TypeError",
    CompressionError::Io(e) => get_io_error_class(e),
  }
}

fn get_web_message_port_error_class(e: &MessagePortError) -> &'static str {
  match e {
    MessagePortError::InvalidTransfer => "TypeError",
    MessagePortError::NotReady => "TypeError",
    MessagePortError::TransferSelf => "TypeError",
    MessagePortError::Canceled(e) => {
      let io_err: io::Error = e.to_owned().into();
      get_io_error_class(&io_err)
    }
    MessagePortError::Resource(e) => get_error_class_name(e).unwrap_or("Error"),
  }
}

fn get_web_stream_resource_error_class(
  e: &StreamResourceError,
) -> &'static str {
  match e {
    StreamResourceError::Canceled(e) => {
      let io_err: io::Error = e.to_owned().into();
      get_io_error_class(&io_err)
    }
    StreamResourceError::Js(_) => "TypeError",
  }
}

fn get_web_blob_error_class(e: &BlobError) -> &'static str {
  match e {
    BlobError::BlobPartNotFound => "TypeError",
    BlobError::SizeLargerThanBlobPart => "TypeError",
    BlobError::BlobURLsNotSupported => "TypeError",
    BlobError::Url(_) => "Error",
  }
}

fn get_ffi_repr_error_class(e: &ReprError) -> &'static str {
  match e {
    ReprError::InvalidOffset => "TypeError",
    ReprError::InvalidArrayBuffer => "TypeError",
    ReprError::DestinationLengthTooShort => "RangeError",
    ReprError::InvalidCString => "TypeError",
    ReprError::CStringTooLong => "TypeError",
    ReprError::InvalidBool => "TypeError",
    ReprError::InvalidU8 => "TypeError",
    ReprError::InvalidI8 => "TypeError",
    ReprError::InvalidU16 => "TypeError",
    ReprError::InvalidI16 => "TypeError",
    ReprError::InvalidU32 => "TypeError",
    ReprError::InvalidI32 => "TypeError",
    ReprError::InvalidU64 => "TypeError",
    ReprError::InvalidI64 => "TypeError",
    ReprError::InvalidF32 => "TypeError",
    ReprError::InvalidF64 => "TypeError",
    ReprError::InvalidPointer => "TypeError",
    ReprError::Permission(e) => get_error_class_name(e).unwrap_or("Error"),
  }
}

fn get_ffi_dlfcn_error_class(e: &DlfcnError) -> &'static str {
  match e {
    DlfcnError::RegisterSymbol { .. } => "Error",
    DlfcnError::Dlopen(_) => "Error",
    DlfcnError::Permission(e) => get_error_class_name(e).unwrap_or("Error"),
    DlfcnError::Other(e) => get_error_class_name(e).unwrap_or("Error"),
  }
}

fn get_ffi_static_error_class(e: &StaticError) -> &'static str {
  match e {
    StaticError::Dlfcn(e) => get_ffi_dlfcn_error_class(e),
    StaticError::InvalidTypeVoid => "TypeError",
    StaticError::InvalidTypeStruct => "TypeError",
    StaticError::Resource(e) => get_error_class_name(e).unwrap_or("Error"),
  }
}

fn get_ffi_callback_error_class(e: &CallbackError) -> &'static str {
  match e {
    CallbackError::Resource(e) => get_error_class_name(e).unwrap_or("Error"),
    CallbackError::Other(e) => get_error_class_name(e).unwrap_or("Error"),
    CallbackError::Permission(e) => get_error_class_name(e).unwrap_or("Error"),
  }
}

fn get_ffi_call_error_class(e: &CallError) -> &'static str {
  match e {
    CallError::IR(_) => "TypeError",
    CallError::NonblockingCallFailure(_) => "Error",
    CallError::InvalidSymbol(_) => "TypeError",
    CallError::Permission(e) => get_error_class_name(e).unwrap_or("Error"),
    CallError::Callback(e) => get_ffi_callback_error_class(e),
  }
}

fn get_webstorage_class_name(e: &WebStorageError) -> &'static str {
  match e {
    WebStorageError::ContextNotSupported => "DOMExceptionNotSupportedError",
    WebStorageError::Sqlite(_) => todo!(),
    WebStorageError::Io(e) => get_io_error_class(e),
    WebStorageError::StorageExceeded => "DOMExceptionQuotaExceededError",
  }
}

fn get_tls_error_class(e: &TlsError) -> &'static str {
  match e {
    TlsError::Rustls(_) => "Error",
    TlsError::UnableAddPemFileToCert(e) => get_io_error_class(e),
    TlsError::CertInvalid
    | TlsError::CertsNotFound
    | TlsError::KeysNotFound
    | TlsError::KeyDecode => "InvalidData",
  }
}

pub fn get_cron_error_class(e: &CronError) -> &'static str {
  match e {
    CronError::Resource(e) => {
      deno_core::error::get_custom_error_class(e).unwrap_or("Error")
    }
    CronError::NameExceeded(_) => "TypeError",
    CronError::NameInvalid => "TypeError",
    CronError::AlreadyExists => "TypeError",
    CronError::TooManyCrons => "TypeError",
    CronError::InvalidCron => "TypeError",
    CronError::InvalidBackoff => "TypeError",
    CronError::AcquireError(_) => "Error",
    CronError::Other(e) => get_error_class_name(e).unwrap_or("Error"),
  }
}

fn get_canvas_error(e: &CanvasError) -> &'static str {
  match e {
    CanvasError::UnsupportedColorType(_) => "TypeError",
    CanvasError::Image(_) => "Error",
  }
}

pub fn get_cache_error(error: &CacheError) -> &'static str {
  match error {
    CacheError::Sqlite(_) => "Error",
    CacheError::JoinError(_) => "Error",
    CacheError::Resource(err) => {
      deno_core::error::get_custom_error_class(err).unwrap_or("Error")
    }
    CacheError::Other(e) => get_error_class_name(e).unwrap_or("Error"),
    CacheError::Io(err) => get_io_error_class(err),
  }
}

fn get_broadcast_channel_error(error: &BroadcastChannelError) -> &'static str {
  match error {
    BroadcastChannelError::Resource(err) => {
      deno_core::error::get_custom_error_class(err).unwrap()
    }
    BroadcastChannelError::MPSCSendError(_) => "Error",
    BroadcastChannelError::BroadcastSendError(_) => "Error",
    BroadcastChannelError::Other(err) => {
      get_error_class_name(err).unwrap_or("Error")
    }
  }
}

fn get_kv_error(error: &KvError) -> &'static str {
  match error {
    KvError::DatabaseHandler(e) | KvError::Resource(e) | KvError::Kv(e) => {
      get_error_class_name(e).unwrap_or("Error")
    }
    KvError::TooManyRanges(_) => "TypeError",
    KvError::TooManyEntries(_) => "TypeError",
    KvError::TooManyChecks(_) => "TypeError",
    KvError::TooManyMutations(_) => "TypeError",
    KvError::TooManyKeys(_) => "TypeError",
    KvError::InvalidLimit => "TypeError",
    KvError::InvalidBoundaryKey => "TypeError",
    KvError::KeyTooLargeToRead(_) => "TypeError",
    KvError::KeyTooLargeToWrite(_) => "TypeError",
    KvError::TotalMutationTooLarge(_) => "TypeError",
    KvError::TotalKeyTooLarge(_) => "TypeError",
    KvError::Io(e) => get_io_error_class(e),
    KvError::QueueMessageNotFound => "TypeError",
    KvError::StartKeyNotInKeyspace => "TypeError",
    KvError::EndKeyNotInKeyspace => "TypeError",
    KvError::StartKeyGreaterThanEndKey => "TypeError",
    KvError::InvalidCheck(e) => match e {
      KvCheckError::InvalidVersionstamp => "TypeError",
      KvCheckError::Io(e) => get_io_error_class(e),
    },
    KvError::InvalidMutation(e) => match e {
      KvMutationError::BigInt(_) => "Error",
      KvMutationError::Io(e) => get_io_error_class(e),
      KvMutationError::InvalidMutationWithValue(_) => "TypeError",
      KvMutationError::InvalidMutationWithoutValue(_) => "TypeError",
    },
    KvError::InvalidEnqueue(e) => get_io_error_class(e),
    KvError::EmptyKey => "TypeError",
    KvError::ValueTooLarge(_) => "TypeError",
    KvError::EnqueuePayloadTooLarge(_) => "TypeError",
    KvError::InvalidCursor => "TypeError",
    KvError::CursorOutOfBounds => "TypeError",
    KvError::InvalidRange => "TypeError",
  }
}

fn get_net_error(error: &NetError) -> &'static str {
  match error {
    NetError::ListenerClosed => "BadResource",
    NetError::ListenerBusy => "Busy",
    NetError::SocketClosed => "BadResource",
    NetError::SocketClosedNotConnected => "NotConnected",
    NetError::SocketBusy => "Busy",
    NetError::Io(e) => get_io_error_class(e),
    NetError::AcceptTaskOngoing => "Busy",
    NetError::RootCertStore(e)
    | NetError::Permission(e)
    | NetError::Resource(e) => get_error_class_name(e).unwrap_or("Error"),
    NetError::NoResolvedAddress => "Error",
    NetError::AddrParse(_) => "Error",
    NetError::Map(e) => get_net_map_error(e),
    NetError::Canceled(e) => {
      let io_err: io::Error = e.to_owned().into();
      get_io_error_class(&io_err)
    }
    NetError::DnsNotFound(_) => "NotFound",
    NetError::DnsNotConnected(_) => "NotConnected",
    NetError::DnsTimedOut(_) => "TimedOut",
    NetError::Dns(_) => "Error",
    NetError::UnsupportedRecordType => "NotSupported",
    NetError::InvalidUtf8(_) => "InvalidData",
    NetError::UnexpectedKeyType => "Error",
    NetError::InvalidHostname(_) => "TypeError",
    NetError::TcpStreamBusy => "Busy",
    NetError::Rustls(_) => "Error",
    NetError::Tls(e) => get_tls_error_class(e),
    NetError::ListenTlsRequiresKey => "InvalidData",
    NetError::Reunite(_) => "Error",
  }
}

fn get_net_map_error(error: &deno_net::io::MapError) -> &'static str {
  match error {
    deno_net::io::MapError::Io(e) => get_io_error_class(e),
    deno_net::io::MapError::NoResources => "Error",
  }
}

pub fn get_error_class_name(e: &AnyError) -> Option<&'static str> {
  deno_core::error::get_custom_error_class(e)
    .or_else(|| deno_webgpu::error::get_error_class_name(e))
    .or_else(|| deno_websocket::get_network_error_class_name(e))
    .or_else(|| e.downcast_ref::<WebError>().map(get_web_error_class))
    .or_else(|| {
      e.downcast_ref::<CompressionError>()
        .map(get_web_compression_error_class)
    })
    .or_else(|| {
      e.downcast_ref::<MessagePortError>()
        .map(get_web_message_port_error_class)
    })
    .or_else(|| {
      e.downcast_ref::<StreamResourceError>()
        .map(get_web_stream_resource_error_class)
    })
    .or_else(|| e.downcast_ref::<BlobError>().map(get_web_blob_error_class))
    .or_else(|| e.downcast_ref::<IRError>().map(|_| "TypeError"))
    .or_else(|| e.downcast_ref::<ReprError>().map(get_ffi_repr_error_class))
    .or_else(|| {
      e.downcast_ref::<DlfcnError>()
        .map(get_ffi_dlfcn_error_class)
    })
    .or_else(|| {
      e.downcast_ref::<StaticError>()
        .map(get_ffi_static_error_class)
    })
    .or_else(|| {
      e.downcast_ref::<CallbackError>()
        .map(get_ffi_callback_error_class)
    })
    .or_else(|| e.downcast_ref::<CallError>().map(get_ffi_call_error_class))
    .or_else(|| e.downcast_ref::<TlsError>().map(get_tls_error_class))
    .or_else(|| e.downcast_ref::<CronError>().map(get_cron_error_class))
    .or_else(|| e.downcast_ref::<CanvasError>().map(get_canvas_error))
    .or_else(|| e.downcast_ref::<CacheError>().map(get_cache_error))
    .or_else(|| e.downcast_ref::<KvError>().map(get_kv_error))
    .or_else(|| e.downcast_ref::<NetError>().map(get_net_error))
    .or_else(|| {
      e.downcast_ref::<deno_net::io::MapError>()
        .map(get_net_map_error)
    })
    .or_else(|| {
      e.downcast_ref::<BroadcastChannelError>()
        .map(get_broadcast_channel_error)
    })
    .or_else(|| {
      e.downcast_ref::<WebStorageError>()
        .map(get_webstorage_class_name)
    })
    .or_else(|| {
      e.downcast_ref::<deno_url::UrlPatternError>()
        .map(|_| "TypeError")
    })
    .or_else(|| {
      e.downcast_ref::<dlopen2::Error>()
        .map(get_dlopen_error_class)
    })
    .or_else(|| e.downcast_ref::<hyper::Error>().map(get_hyper_error_class))
    .or_else(|| {
      e.downcast_ref::<hyper_util::client::legacy::Error>()
        .map(get_hyper_util_error_class)
    })
    .or_else(|| {
      e.downcast_ref::<hyper_v014::Error>()
        .map(get_hyper_v014_error_class)
    })
    .or_else(|| {
      e.downcast_ref::<Arc<hyper_v014::Error>>()
        .map(|e| get_hyper_v014_error_class(e))
    })
    .or_else(|| {
      e.downcast_ref::<deno_core::Canceled>().map(|e| {
        let io_err: io::Error = e.to_owned().into();
        get_io_error_class(&io_err)
      })
    })
    .or_else(|| {
      e.downcast_ref::<env::VarError>()
        .map(get_env_var_error_class)
    })
    .or_else(|| e.downcast_ref::<io::Error>().map(get_io_error_class))
    .or_else(|| {
      e.downcast_ref::<ModuleResolutionError>()
        .map(get_module_resolution_error_class)
    })
    .or_else(|| {
      e.downcast_ref::<notify::Error>()
        .map(get_notify_error_class)
    })
    .or_else(|| e.downcast_ref::<regex::Error>().map(get_regex_error_class))
    .or_else(|| {
      e.downcast_ref::<serde_json::error::Error>()
        .map(get_serde_json_error_class)
    })
    .or_else(|| {
      e.downcast_ref::<url::ParseError>()
        .map(get_url_parse_error_class)
    })
    .or_else(|| {
      e.downcast_ref::<deno_kv::sqlite::SqliteBackendError>()
        .map(|_| "TypeError")
    })
    .or_else(|| {
      #[cfg(unix)]
      let maybe_get_nix_error_class =
        || e.downcast_ref::<nix::Error>().map(get_nix_error_class);
      #[cfg(not(unix))]
      let maybe_get_nix_error_class = || Option::<&'static str>::None;
      (maybe_get_nix_error_class)()
    })
}

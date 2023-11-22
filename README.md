# Embeddable Deno

> Embeddable Deno is part of the
> [Metatype ecosystem](https://github.com/metatypedev/metatype). Consider
> checking out how this component integrates with the whole ecosystem and browse
> the
> [documentation](https://metatype.dev?utm_source=github&utm_medium=readme&utm_campaign=deno)
> to see more examples.

## Syncing upstream version

```bash
git remote add upstream git@github.com:denoland/deno.git

git checkout PREV_TAG-embeddable
git checkout -b TAG-embeddable
git pull --rebase upstream TAG
# resolve conflict and reset lockfile
git checkout HEAD -- Cargo.lock
git push origin TAG-embeddable
```

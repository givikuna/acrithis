# CLI

## Basic Commands

```
acrithis init
acrithis add <file.lean> --name NAME --type axiom|theorem \
    --dep <hash>[:role] ... [--doc <file.md>] [--tag TAG]...
acrithis get <hash>
acrithis list [--type ...] [--tag ...]
acrithis deps <hash>
acrithis verify <hash>
acrithis verify-all
acrithis graph [--format dot]
```

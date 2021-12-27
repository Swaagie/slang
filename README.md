# Slang CLI

Acronym lookup Command Line Interface tool.

### Install

```sh
cargo install slang-cli
```

### Configuration

Create a `.slang.toml` in your home folder with custom JSON sources. By default NASA and triathlon acronyms are configured.

```toml
[sources]
IT="https://raw.githubusercontent.com/url/to/your/list/of/acronyms.json"
```

The JSON format of an acronym is defined as:

```
{
	acronym_id: Option<u32>
	abbreviation: String
	expansion: String
}
```

### Usage

Search all configured sources:

```sh
slang [FLAGS] [OPTIONS] <acronym>
```

To filter for specific context or jargon:

```sh
slang --context NASA <acronym>
```

Example:

```sh
slang --context NASA RPM

Found the following expansions:

✅ Context NASA: Random Predictors Mode
```
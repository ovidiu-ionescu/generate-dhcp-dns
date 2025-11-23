# orgncf-generator

Can generate a DNS zone file a DHCP ip reservation file starting from a
orgncf configuration file.

The parser for the orgncf configuration file is implemente with tree-sitter
and is hoster [here](https://github.com/ovidiu-ionescu/tree-sitter-orgncf).

Before the generation, it performs some validation steps to ensure that 
there are no duplicate MAC addresses, IP addresses or hostnames.

## Usage

```bash
orgncf-generator -i <input.ncf> -o output_dir
```

The names of the files generated are specified in the configuration file.


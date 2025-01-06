use clap::{Parser, Subcommand};

use std::fs::{read_to_string, write};
use std::path::Path;

#[derive(Parser)]
#[command(version, name = "cargo-crest", about = "A helper utility for crest")]
struct Cli {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a css property file for crest to extend its functionality
    GenProperty {
        /// The name of the (first) css property
        #[arg(name = "name")]
        property_name: String,

        /// Additional properties to generate with "name". Shares flags with "name".
        #[arg(long, value_delimiter = ',')]
        extra_property_names: Vec<String>,

        /// This is the name used for the output file and shared keyword type (if any)
        #[arg(long)]
        shared_name: Option<String>,

        /// Specify that the css property has no keywords
        #[arg(long)]
        no_keywords: bool,

        /// What keywords do the properties use?
        #[arg(long, value_delimiter = ',')]
        keywords: Vec<String>,

        /// What are the valid parser token types?
        /// Valid options are: ident, hash, quoted-string,
        /// unquoted-url, number, percentage, dimension,
        /// function
        /// (separate values using a comma
        /// (eg: "ident,hash,number"))
        #[arg(long, value_delimiter = ',')]
        tokens: Vec<String>,

        /// do not write to file - instead, print to terminal
        #[arg(short, long)]
        dry_run: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.subcommand {
        Commands::GenProperty {
            property_name,
            extra_property_names,
            shared_name,
            no_keywords,
            keywords,
            tokens,
            dry_run,
        } => {
            let mut source = String::new();

            assert!(
                tokens.len() > 0,
                "At least one token type must be provided using '--tokens'"
            );

            fn dash_to_camel(dashed: &str) -> String {
                let mut prev_underscore = false;
                let mut result = String::new();

                let mut chars = dashed.chars();
                result += &chars.next().unwrap().to_uppercase().to_string();

                for char in chars {
                    if char == '-' {
                        prev_underscore = true;
                    } else if prev_underscore {
                        result += &char.to_uppercase().to_string();
                        prev_underscore = false;
                    } else {
                        result += &char.to_string();
                    }
                }

                result
            }

            source += "use super::{CssValue, TokenExpected};\n";
            source += "use crate::Unit;\n\n";

            let camel_attr_name = format!("Css{}", dash_to_camel(&property_name));

            source += "#[derive(Debug, Clone)]\n";
            source += &format!("pub struct {}(Unit);\n\n", camel_attr_name);

            let shared_name = shared_name.unwrap_or(property_name.clone());

            for name in extra_property_names.iter() {
                source += "#[derive(Debug, Clone)]\n";
                source += &format!("pub struct Css{}(Unit);\n\n", dash_to_camel(&*name));
            }

            let keyword = if !no_keywords {
                let kw_value = format!("Keyword{}", dash_to_camel(&shared_name));

                source +=
                    "#[derive(Debug, Clone, strum_macros::EnumString, strum_macros::Display)]\n";
                source += &format!("pub enum {kw_value} {{\n");

                for keyword in keywords.into_iter() {
                    source += &format!(
                        "    #[strum(to_string = \"{keyword}\", serialize = \"{keyword}\")]\n"
                    );
                    source += &format!("    {},\n\n", dash_to_camel(&keyword));
                }

                source += "}\n";

                kw_value
            } else {
                "super::KeywordNone".into()
            };

            source += &format!(
                "
impl From<Unit> for {camel_attr_name} {{
    fn from(value: Unit) -> Self {{
        Self(value)
    }}
}}
            
impl Into<Unit> for {camel_attr_name} {{
    fn into(self) -> Unit {{
        self.0
    }}
}}
"
            );

            for name in extra_property_names.iter() {
                let camel_name = dash_to_camel(&*name);
                source += &format!(
                    "
impl From<Unit> for Css{camel_name} {{
    fn from(value: Unit) -> Self {{
        Self(value)
    }}
}}
            
impl Into<Unit> for Css{camel_name} {{
    fn into(self) -> Unit {{
        self.0
    }}
}}
"
                );
            }

            static TOKEN_TYPES: [&'static str; 8] = [
                "ident",
                "hash",
                "quoted-string",
                "unquoted-url",
                "number",
                "percentage",
                "dimension",
                "function",
            ];
            for token in tokens.iter() {
                assert!(
                    TOKEN_TYPES.contains(&(token as &str)),
                    "token value '{token}' is not a valid token type"
                );
            }

            let tokens = tokens
                .iter()
                .map(|x| format!("TokenExpected::{}", dash_to_camel(&*x)))
                .collect::<Vec<_>>();

            source += &format!(
                "
impl CssValue for {camel_attr_name} {{
    type Keyword = {keyword};

    fn type_name() -> &'static str {{
        \"{camel_attr_name}\"
    }}

    fn type_token() -> TokenExpected {{
        {}
    }}
}}
",
                tokens.join(" | ")
            );

            for name in extra_property_names.iter() {
                let camel_name = format!("Css{}", dash_to_camel(&*name));
                source += &format!(
                    "
impl CssValue for {camel_name} {{
    type Keyword = {keyword};

    fn type_name() -> &'static str {{
        \"{camel_name}\"
    }}

    fn type_token() -> TokenExpected {{
        {}
    }}
}}
",
                    tokens.join(" | ")
                );
            }

            let filename = format!("{}.rs", shared_name.replace("-", "_"));

            fn dir_is_crest(path: &Path) -> bool {
                let manifest_path = path.join(Path::new("Cargo.toml"));
                let manifest_contents = read_to_string(manifest_path).unwrap();
                let manifest_toml = manifest_contents
                    .parse::<toml::Table>()
                    .expect(&format!("'{path:?}' is an invalid TOML file!"));

                let package_name = manifest_toml["package"].as_table().unwrap()["name"]
                    .as_str()
                    .unwrap();

                let attr_path = Path::new("src/style/parse_attr");

                // if package_name == "peacock-crest" {
                //     let result_buf = path.join(attr_path);
                //     let result_dir = result_buf.as_path();
                //     if result_dir.is_dir() {
                //         true
                //     }
                //     else {
                //         false
                //     }
                // }
                // else {
                //     false
                // }
                package_name == "peacock-crest" && path.join(attr_path).is_dir()
            }

            let attr_path = Path::new("src/style/parse_attr");
            for manifest_result in glob::glob("**/Cargo.toml").expect("Failed to read glob pattern")
            {
                let manifest_pathbuf =
                    manifest_result.expect("Something went wrong while iterating over the glob");
                let manifest = manifest_pathbuf.as_path().parent().unwrap();
                if dir_is_crest(manifest) {
                    let full_attr_path = manifest.join(attr_path).join(Path::new(&filename));
                    if !dry_run {
                        write(full_attr_path.clone(), source)
                            .expect(&format!("Failed to write results to '{full_attr_path:?}'"));
                    } else {
                        println!("{full_attr_path:?}:\n=================================\n{source}\n=================================")
                    }
                    break;
                }
            }
        }
    }
}

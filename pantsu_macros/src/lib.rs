use proc_macro::TokenStream;
use std::fs;

use lz_fnv::{Fnv1a, FnvHasher};
use quote::quote;
use regex::Regex;
use syn::{LitStr, parse_macro_input};

const FILENAME_REGEX: &str = r"(?<version>v(?<major_version>\d+)\.(?<minor_version>\d+)\.(?<patch_version>\d+))__(?<description>[^.]+)\.sql";

#[proc_macro]
pub fn include_migrations(input: TokenStream) -> TokenStream {
    let directory = parse_macro_input!(input as LitStr).value();
    let filename_regex = Regex::new(FILENAME_REGEX).unwrap();
    let dir_content = fs::read_dir(&directory)
        .expect(format!("cannot read directory {}", directory).as_str());
    //let mut files: Vec<regex::Captures> = Vec::new();
    let mut sortable_migrations = dir_content.flat_map(|dir_entry| {
        let dir_entry = dir_entry.ok()?;
        let filepath = dir_entry.path();
        let filename = dir_entry.file_name();
        let filename_str = filename.to_str().unwrap();
        let captures = filename_regex.captures(filename_str)?;
        let content = fs::read_to_string(filepath)
            .expect(format!("cannot read file {}", captures.get(0).unwrap().as_str()).as_str());
        let migration = file_to_migration(&captures["version"], &captures["description"], content);
        Some((
            captures["major_version"].parse::<u32>().unwrap(),
            captures["minor_version"].parse::<u32>().unwrap(),
            captures["patch_version"].parse::<u32>().unwrap(),
            migration,
        ))
    }).collect::<Vec<(u32, u32, u32, proc_macro2::TokenStream)>>();
    sortable_migrations.sort_by_key(|migration| (migration.0, migration.1, migration.2));
    let migrations = sortable_migrations.into_iter().map(|m| m.3).collect::<Vec<proc_macro2::TokenStream>>();
    quote!(
        vec![
            #(#migrations),*
        ]
    ).into()
}

fn file_to_migration(version: &str, description: &str, file_content: String) -> proc_macro2::TokenStream {
    let mut hasher = Fnv1a::<u64>::new();
    hasher.write(file_content.as_bytes());
    let hash = format!("{:016x}", hasher.finish());
    quote!(
        Migration {
            version: #version.to_owned(),
            description: #description.to_owned(),
            hash: #hash.to_owned(),
            sql: #file_content.to_owned(),
        }
    )
}

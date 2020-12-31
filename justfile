badge-crates := "[![crate](https://img.shields.io/crates/v/easy_collections)](https://crates.io/crates/easy_collections)"
badge-docs := "[![documentation](https://docs.rs/easy_collections/badge.svg)](https://docs.rs/easy_collections)"

readme:
	printf "%s\n%s\n%s" "{{ badge-crates }}" "{{ badge-docs }}" "$(cargo readme)" > README.md
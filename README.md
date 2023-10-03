# Rust-Testbed
Rust Testbed for keep all files in one folder, run it. <br/>
### How to use?
1. ```cargo new "create_new_package" --lib```
2. make "tests" folder at the root directory
3. create .rs files and done<br/>
※ Duplicate "00_default.rs"file in "tests" folder to create new rust test file.
```
//to run
cargo test --test <your file name> -- --nocapture
```

Tests 폴더 안에서 새로운 .rs 파일 만들어서 사용 <br/>
(00_default.rs에 있는 내용 복사, 붙여넣기)

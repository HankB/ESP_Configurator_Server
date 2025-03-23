# database library

With apologies to users of OSs that can't differentiate between upper and lower case. But `../DB` was already there (for scripts and database storage) and w/out much thought I used `../db` in another project to develop a Rust library to manage database access.

The code here waas explored/developed in <https://github.com/HankB/Fun_with_rusqlite/tree/main> and the library file(s) will simply be copied here. from the `w3resource` subdirectory. In that project I deleted the `db/Cargo.toml` fine because it seemed unnecessary but I will leave it here in case it serves some purpose.

## 2025-03-23 adding the database lib

```text
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server$ git status
On branch main
Your branch is up to date with 'origin/main'.

Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        new file:   db/Cargo.toml
        new file:   db/src/lib.rs

hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server$ git commit -m "adding db library"
[main 0d58471] adding db library
 2 files changed, 20 insertions(+)
 create mode 100644 db/Cargo.toml
 create mode 100644 db/src/lib.rs
hbarta@olive:~/Programming/ESP_Conf/ESP_Configurator_Server$ 
```

(And then created this README.md to record this.)

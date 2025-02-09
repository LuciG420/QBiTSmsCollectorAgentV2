# QBiTSmsCollectorAgentV2
Ink.qbit.smscollector.app

my_project/
├── .gitignore
├── Cargo.toml
├── build.rs
├── src/
│   ├── config/
│   │   ├── mod.rs
│   │   ├── caddy.rs
│   ├── enclave/
│   │   ├── mod.rs
│   │   ├── secrets.rs
│   ├── event/
│   │   ├── mod.rs
│   │   ├── handlers.rs
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── sqlite.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── lua_executor.rs
│   ├── main.rs
│   ├── events.proto
├── tests/
│   ├── integration_tests.rs
├── ui/
│   ├── admin_app/
│   │   ├── src/
│   │   │   ├── App.js
│   │   │   ├── components/
│   │   │   ├── __tests__/
│   │   ├── package.json
│   │   ├── yarn.lock
│   │   ├── .gitignore
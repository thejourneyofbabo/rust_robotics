# Rust Robotics

Rust로 구현하는 로보틱스 알고리즘 라이브러리

## 프로젝트 구조

```
rust_robotics/
├── Cargo.toml
├── src/
│   ├── lib.rs                  # 라이브러리 진입점
│   ├── bin/                    # 실행 파일 (cargo run --bin)
│   │   ├── planner.rs
│   │   └── tester.rs
│   ├── path_planning.rs        # 경로 계획 모듈
│   ├── path_planning/
│   │   └── demo_planner.rs
│   ├── utils.rs                # 유틸리티 모듈
│   └── utils/
│       └── math.rs
└── examples/                   # 데모/테스트 (cargo run --example)
    ├── demo.rs
    └── quick_test.rs
```

## 실행

```bash
# bin (실행 파일)
cargo run --bin planner
cargo run --bin tester

# examples (데모/테스트)
cargo run --example demo
cargo run --example quick_test
```

## bin vs examples

| | `src/bin/` | `examples/` |
|---|------------|-------------|
| 용도 | 완성된 프로그램 | 테스트/데모 코드 |
| cargo build | 자동 빌드 | 빌드 안됨 |
| 배포 | 포함됨 | 포함 안됨 |

## 개발 가이드

### 새 알고리즘 추가

1. `src/path_planning/dijkstra.rs` 생성
2. `src/path_planning.rs`에 `pub mod dijkstra;` 추가

### 새 bin 추가

`src/bin/visualizer.rs` 생성 → `cargo run --bin visualizer`

### 새 example 추가

`examples/dijkstra_demo.rs` 생성 → `cargo run --example dijkstra_demo`

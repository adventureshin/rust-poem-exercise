# Poem API with SeaORM

Rust 기반 REST API 서버 프로젝트입니다.

## 원본 템플릿

이 프로젝트는 [jonathanblade/poem-api](https://github.com/jonathanblade/poem-api.git)를 기반으로 시작했습니다.

### 변경 사항

| 항목 | 원본 | 현재 |
|------|------|------|
| 웹 프레임워크 | Poem | Poem |
| ORM | SQLx (raw SQL) | SeaORM |
| 데이터베이스 | SQLite | PostgreSQL |
| 마이그레이션 | SQLx migrate | SeaORM migration |

## 프로젝트 구조

```
.
├── migration/                # SeaORM 마이그레이션 (별도 crate)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── main.rs
│       └── m20220101_000001_create_users.rs
├── scripts/                  # 유틸리티 스크립트
│   ├── generate-entity.sh    # Entity 자동 생성
│   ├── generate-migration.sh # 마이그레이션 파일 생성
│   └── migrate.sh            # 마이그레이션 실행
├── src/
│   ├── common/               # 공통 유틸리티, 에러 처리
│   ├── config.rs             # 환경 설정
│   ├── context.rs            # 앱 컨텍스트 (DB 연결)
│   ├── controller/           # API 컨트롤러
│   ├── db/                   # 데이터베이스 연결 및 레포지토리
│   │   └── repo/             # Repository 패턴 구현
│   ├── entity/               # SeaORM Entity (자동 생성)
│   ├── middleware/           # 미들웨어
│   ├── response/             # API 응답 타입
│   ├── scheme/               # 요청/응답 DTO
│   ├── service/              # 비즈니스 로직
│   ├── lib.rs
│   └── main.rs
└── tests/                    # 통합 테스트
```

## 사전 준비

### 1. CLI 도구 설치

```bash
# SeaORM CLI (마이그레이션, Entity 생성)
cargo install sea-orm-cli

# cargo-watch (파일 변경 시 자동 재빌드)
cargo install cargo-watch
```

### 2. 환경 변수 설정

프로젝트 루트에 `.env` 파일을 생성합니다:

```env
DATABASE_SERVER=localhost
DATABASE_USER=your_user
DATABASE_PASSWORD=your_password
DATABASE_DB=your_database
DATABASE_PORT=3000
```

### 3. 스크립트 실행 권한 부여

```bash
chmod +x scripts/*.sh
```

## 스크립트 사용법

### 마이그레이션 실행

```bash
# 마이그레이션 적용
./scripts/migrate.sh up

# 마이그레이션 롤백
./scripts/migrate.sh down

# 마이그레이션 상태 확인
./scripts/migrate.sh status

# 전체 초기화 후 재실행
./scripts/migrate.sh fresh
```

### 마이그레이션 파일 생성

```bash
./scripts/generate-migration.sh create_posts_table
```

`migration/src/` 디렉토리에 새 마이그레이션 파일이 생성됩니다.

### Entity 자동 생성

DB 스키마를 기반으로 Entity 파일을 자동 생성합니다:

```bash
./scripts/generate-entity.sh
```

`src/entity/` 디렉토리에 Entity 파일이 생성됩니다.

## 실행

```bash
# 개발 서버 실행
cargo run

# 개발 서버 실행 (파일 변경 시 자동 재시작)
cargo watch -x run

# 릴리즈 빌드
cargo build --release
```

## API 문서

서버 실행 후 Swagger UI에서 API 문서를 확인할 수 있습니다:

```
http://localhost:3000/
```

## 주요 의존성

| 패키지 | 용도 |
|--------|------|
| poem | 웹 프레임워크 |
| poem-openapi | OpenAPI/Swagger 지원 |
| sea-orm | ORM |
| sea-orm-migration | 마이그레이션 |
| tokio | 비동기 런타임 |
| serde | 직렬화/역직렬화 |
| jsonwebtoken | JWT 인증 |
| bcrypt | 비밀번호 해싱 |

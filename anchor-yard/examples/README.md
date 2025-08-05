# Anchor Yard Examples

이 폴더에는 anchor-yard 라이브러리의 다양한 사용법을 보여주는 예제들이 있습니다.

## 예제 목록

### 1. `basic_usage.rs` - 기본 사용법
가장 기본적인 시스템 스냅샷 기능을 보여줍니다.

```bash
cargo run --example basic_usage
```

**주요 기능:**
- `#[snapshot_system]` 매크로 사용법
- threshold 설정
- 개별 시스템 및 workload 실행
- 자동 스냅샷 파일 생성

## 생성되는 파일들

각 예제를 실행하면 `snapshots/` 폴더에 다음과 같은 파일들이 생성됩니다:

```
snapshots/
├── slow_combat_system_1703123456.snapshot
├── heavy_ai_system_1703123457.snapshot
└── rendering_system_1703123458.snapshot
```

## 스냅샷 파일 구조

각 `.snapshot` 파일에는 다음 정보가 포함됩니다:
- 시스템 이름
- 실행 시간 (ms)
- 타임스탬프
- World 상태 (직렬화된 바이너리 데이터)

## 사용 팁

1. **threshold 조정**: 시스템의 복잡도에 따라 적절한 threshold를 설정하세요
2. **스냅샷 관리**: 오래된 스냅샷은 정기적으로 정리하세요
3. **벤치마킹**: 최적화 전후 성능을 비교할 때 동일한 스냅샷을 사용하세요

## 주의사항

- 스냅샷 파일은 바이너리 형태로 저장되어 크기가 클 수 있습니다
- development 환경에서만 사용하고 production에서는 비활성화하세요
- 복잡한 World 상태일수록 직렬화/역직렬화 시간이 늘어날 수 있습니다

## 문제 해결

### 스냅샷이 생성되지 않음
- threshold가 너무 높게 설정되어 있는지 확인
- 시스템이 실제로 threshold를 넘는지 확인 (의도적으로 딜레이 추가해보기)

### 메모리 사용량이 높음
- 큰 World 상태를 자주 직렬화하면 메모리 사용량이 증가할 수 있습니다
- threshold를 높여서 스냅샷 빈도를 줄이거나, 오래된 스냅샷을 정리하세요
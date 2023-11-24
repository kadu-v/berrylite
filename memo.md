## Hello World
0 (id=0): size=16, offset=0, first_used=0 last_used=1
1 (id=1): size=64, offset=64, first_used=1 last_used=2
2 (id=2): size=64, offset=0, first_used=2 last_used=3
3 (id=3): size=16, offset=64, first_used=3 last_used=3

## Person detection
0 (id=0): size=256, offset=2304, first_used=28 last_used=29
1 (id=1): size=16, offset=0, first_used=29 last_used=30
2 (id=2): size=16, offset=16, first_used=30 last_used=31
3 (id=3): size=18432, offset=0, first_used=1 last_used=2
4 (id=4): size=4608, offset=4608, first_used=20 last_used=21
5 (id=5): size=4608, offset=0, first_used=21 last_used=22
6 (id=6): size=4608, offset=4608, first_used=22 last_used=23
7 (id=7): size=4608, offset=0, first_used=23 last_used=24
8 (id=8): size=1152, offset=4608, first_used=24 last_used=25
9 (id=9): size=2304, offset=0, first_used=25 last_used=26
a (id=10): size=2304, offset=2304, first_used=26 last_used=27
b (id=11): size=2304, offset=0, first_used=27 last_used=28
c (id=12): size=18432, offset=36864, first_used=2 last_used=3
d (id=13): size=36864, offset=0, first_used=3 last_used=4
e (id=14): size=9216, offset=36864, first_used=4 last_used=5
f (id=15): size=18432, offset=0, first_used=5 last_used=6
g (id=16): size=18432, offset=18432, first_used=6 last_used=7
h (id=17): size=18432, offset=0, first_used=7 last_used=8
i (id=18): size=4608, offset=18432, first_used=8 last_used=9
j (id=19): size=9216, offset=0, first_used=9 last_used=10
k (id=20): size=9216, offset=9216, first_used=10 last_used=11
l (id=21): size=9216, offset=0, first_used=11 last_used=12
m (id=22): size=2304, offset=9216, first_used=12 last_used=13
n (id=23): size=4608, offset=0, first_used=13 last_used=14
o (id=24): size=4608, offset=4608, first_used=14 last_used=15
p (id=25): size=4608, offset=0, first_used=15 last_used=16
q (id=26): size=4608, offset=4608, first_used=16 last_used=17
r (id=27): size=4608, offset=0, first_used=17 last_used=18
s (id=28): size=4608, offset=4608, first_used=18 last_used=19
t (id=29): size=4608, offset=0, first_used=19 last_used=20
u (id=30): size=16, offset=0, first_used=31 last_used=31
v (id=31): size=9216, offset=18432, first_used=0 last_used=1


## メモリ最適化
[TfLiteStatus MicroAllocator::CommitStaticMemoryPlan](https://vscode.dev/github/kadu-v/tflite-micro-sample/blob/main/tensorflow/lite/micro/micro_allocator.cc#L802)
ここで，メモリ配置の最適化をする．


[TfLiteStatus AllocationInfoBuilder::CreateAllocationInfo](https://vscode.dev/github/kadu-v/tflite-micro-sample/blob/main/tensorflow/lite/micro/micro_allocation_info.cc#L103)
アロケーション情報の構造体を作成．

[builder.GetOfflinePlannedOffsets(&offline_planner_offsets));](https://vscode.dev/github/kadu-v/tflite-micro-sample/blob/main/tensorflow/lite/micro/micro_allocator.cc#L824)
オフラインプランをしていたら，存在するけど，デフォルトはない．
よって関係なし．


[TfLiteStatus AllocationInfoBuilder::InitializeAllocationInfo(](https://vscode.dev/github/kadu-v/tflite-micro-sample/blob/main/tensorflow/lite/micro/micro_allocation_info.cc#L178)
allocation infoの初期化ここで，タイムスタンプとかの初期化をしている．

[TfLiteStatus AllocationInfoBuilder::MarkAllocationLifetimes(](https://vscode.dev/github/kadu-v/tflite-micro-sample/blob/main/tensorflow/lite/micro/micro_allocation_info.cc#L233-L234)
ここで，書くオペレーションのタイムタンプを作成している．

[CreatePlan(memory_planner_, allocation_info, allocation_info_count));](https://vscode.dev/github/kadu-v/tflite-micro-sample/blob/main/tensorflow/lite/micro/micro_allocator.cc#L859-L860)
ここで，メモリアロケーションプランを作成している．


[https://vscode.dev/github/kadu-v/tflite-micro-sample/blob/main/tensorflow/lite/micro/micro_allocation_info.cc#L195-L196](https://vscode.dev/github/kadu-v/tflite-micro-sample/blob/main/tensorflow/lite/micro/micro_allocation_info.cc#L195-L196)
ここで，書くテンソルの先頭ポインタをallocatorが保持している．(まだ，allocation されていない)
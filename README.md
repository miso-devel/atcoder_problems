# 使い方


## ABCする
```
task abc -- abc
```

テスト
```
task abc-test-a -- [abc]
task abc-test-b -- [abc]
task abc-test-c -- [abc]
task abc-test-d -- [abc]
```
例
```
task abc-test-a -- 001
task abc-submit-a -- 001
```

## ARCする
```
task arc -- arc
```

テスト
```
task arc-test-a -- [arc]
task arc-test-b -- [arc]
task arc-test-c -- [arc]
task arc-test-d -- [arc]
```
## エラーハンドリング
```
task abc-test-a -- 001 などを実行しようとするときにIO error for operation on ~/atcoder/ABC/abc006/testcases/c: No such file or directory (os error 2) が出た場合は、実行しようとしているテストのtestcase配下にフォルダが足りていないので/aフォルダなど追加する
```

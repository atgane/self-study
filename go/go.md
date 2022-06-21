# 배열
`[]int` -> 슬라이스: 동적 배열. 길이가 고정되어있지 않다. 
`[...]int{1, 2, 3}` -> 배열: 길이가 정해져 있는 배열. 

배열 선언시 개수는 항상 상수.

```go
for i, v := range arr {
    //...
}
```

# 구조체
```go
type A struct {
    a string
    b int
    c int 
    d float64
}
```

구조체 포함. embedded field
```go
type User struct {
    Name string
    ID string
    Age int
    Level int 
}

type VIPUser struct {
    User
    VIPLevel int
    Price int
}
```
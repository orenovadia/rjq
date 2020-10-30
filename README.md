```
echo '{"a": {"b": 42} }' | cargo run '.a.b' 2>/dev/null
result: 42
```

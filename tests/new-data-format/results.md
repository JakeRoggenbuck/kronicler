# New Log Format

The old format used a verbose encoding that looked like this:

```json
{
  "id":0,
  "fields":[
    {"type":"Name","value":"get_board_data"},
    {"type":"Epoch","value":1759809763045650181},
    {"type":"Epoch","value":1759809763121428818}
    {"type":"Epoch","value":75778637}
  ]
}
```

But I changed this to be:

```
[0,"get_board_data",1759809763045650181,75778637]
```

This resulted in the 697 logs taking `39k` instead of `133k`. Larger systems may want compressed data as well, and that may be built soon.

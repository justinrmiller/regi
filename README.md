# regi

![regi](regi.png)

## A Rust Regex Command Line Tool for Parquet

To run:
```
cargo run
```

This will by default run the 100 rule config.json file. config_all.json can be renamed to config.json for 1000. 

- 100 rules is ~2.45 s / 10000 rows against the Yelp review dataset (~6.99 million rows, estimated completion time: 28.5 minutes)
- 1000 rules is ~25.2 s / 10000 rows against the Yelp review dataset (~6.99 million rows, estimated completion time: 4.89 hours)

Example output of rule count map:

This was the output at line count 50000:
```json

{
    "text|American": 504,
    "text|a": 49994,
    "text|ability": 191,
    "text|able": 12183,
    "text|about": 8984,
    "text|above": 629,
    "text|accept": 452,
    "text|according": 136,
    "text|account": 168,
    "text|across": 885,
    "text|act": 6076,
    "text|action": 468,
    "text|activity": 49,
    "text|actually": 1897,
    "text|add": 2978,
    "text|address": 196,
    "text|administration": 1,
    "text|admit": 310,
    "text|adult": 310,
    "text|affect": 53,
    "text|after": 5486,
    "text|again": 5024,
    "text|against": 181,
    "text|age": 7383,
    "text|agency": 8,
    "text|agent": 38,
    "text|ago": 1592,
    "text|agree": 559,
    "text|agreement": 17,
    "text|ahead": 495,
    "text|air": 4766,
    "text|all": 27888,
    "text|allow": 703,
    "text|almost": 1637,
    "text|alone": 453,
    "text|along": 882,
    "text|already": 1015,
    "text|also": 7082,
    "text|although": 845,
    "text|always": 5445,
    "text|among": 225,
    "text|amount": 1182,
    "text|analysis": 5,
    "text|and": 45121,
    "text|animal": 143,
    "text|another": 2933,
    "text|answer": 644,
    "text|any": 10919,
    "text|anyone": 1217,
    "text|anything": 2158,
    "text|appear": 592,
    "text|apply": 47,
    "text|approach": 217,
    "text|area": 4190,
    "text|argue": 87,
    "text|arm": 2915,
    "text|around": 4026,
    "text|arrive": 1531,
    "text|art": 10186,
    "text|article": 35,
    "text|artist": 201,
    "text|as": 42366,
    "text|ask": 4835,
    "text|assume": 222,
    "text|at": 44044,
    "text|attack": 44,
    "text|attention": 651,
    "text|attorney": 11,
    "text|audience": 43,
    "text|author": 43,
    "text|authority": 13,
    "text|available": 990,
    "text|avoid": 451,
    "text|away": 2630,
    "text|baby": 331,
    "text|back": 11530,
    "text|bad": 3136,
    "text|bag": 1045,
    "text|ball": 1060,
    "text|bank": 158,
    "text|bar": 6414,
    "text|base": 954,
    "text|be": 34725,
    "text|beat": 759,
    "text|beautiful": 1451,
    "text|because": 6560,
    "text|become": 403,
    "text|bed": 1168,
    "text|before": 4034,
    "text|begin": 412,
    "text|behavior": 79,
    "text|behind": 871,
    "text|believe": 918,
    "text|benefit": 129,
    "text|best": 6771,
    "text|better": 4661,
    "text|between": 912,
    "text|beyond": 567,
    "text|big": 2838,
    "text|bill": 799
}
```
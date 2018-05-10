## Primitive Map

[![Build Status](https://travis-ci.org/mersinvald/primitive-map-rs.svg?branch=master)](https://travis-ci.org/mersinvald/primitive-map-rs)
[![Docs.rs](https://docs.rs/primitivemap/badge.svg)](https://docs.rs/primitivemap/)
[![Crates.io](https://img.shields.io/crates/d/primitivemap.svg)](https://crates.io/crates/primitivemap)

### Performace

There are 3 testing scenarios of benchmarking maps:
 - Full load: 1 value per bucket
 - Low load: load lower then capacity
 - Overload: load way more then capacity to saturate buckets

|**Full Load [Capacity: 256, Load: 256]**|
|-|
|<img src="https://docs.google.com/spreadsheets/d/e/2PACX-1vSts7sat76z8rw-Dnh9X3JIvrnipPcmOAVrpqRyKmEv-5KteRmzor9LbV-RuCjV5X3FKADXp2Y7w7Tf/pubchart?oid=342491602&format=image" width="100%" height="100%">|

|**Full Load [Capacity: 65535, Load: 65535]**|
|-|
|<img src="https://docs.google.com/spreadsheets/d/e/2PACX-1vSts7sat76z8rw-Dnh9X3JIvrnipPcmOAVrpqRyKmEv-5KteRmzor9LbV-RuCjV5X3FKADXp2Y7w7Tf/pubchart?oid=1974898831&format=image" width="100%" height="100%">|

|**Low Load [Capacity: 1024, Load: 256]**|
|-|
|<img src="https://docs.google.com/spreadsheets/d/e/2PACX-1vSts7sat76z8rw-Dnh9X3JIvrnipPcmOAVrpqRyKmEv-5KteRmzor9LbV-RuCjV5X3FKADXp2Y7w7Tf/pubchart?oid=1287249410&format=image" width="100%" height="100%">|

|**Overload [Capacity: 256, Load: 8192]**|
|-|
|<img src="https://docs.google.com/spreadsheets/d/e/2PACX-1vSts7sat76z8rw-Dnh9X3JIvrnipPcmOAVrpqRyKmEv-5KteRmzor9LbV-RuCjV5X3FKADXp2Y7w7Tf/pubchart?oid=331468148&format=image" width="100%" height="100%">|

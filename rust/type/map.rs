use std::collections::BTreeMap;
use std::collections::HashMap;

fn main() {
	let mut bmap = BTreeMap::new();
	bmap.insert("bkey1", "bvalue1");
	bmap.insert("bkey2", "bvalue2");
	bmap.insert("bkey3", "bvalue3");
	println!("bmap is {:?}", bmap);
	for (k, v) in bmap.iter() {
		println!("bmap: {}={}", k, v);
	}

	let mut ss = String::from("bkey");
	ss += &1.to_string();
	println!("ss={}", ss);
	if let Some(x) = bmap.get(ss.as_str()) {
		println!("{}={}", ss, x);
	} else {
		println!("{} no exist", ss);
	}

	println!("----------------------------------------------------------");

	for i in 0..10 {
		let mut key = "bkey".to_string();
		key += &i.to_string();

		if let Some(x) = bmap.get(key.as_str()) {
			println!("{}={}", key, x);
		} else {
			println!("{} not exist", key);
		}
	}
	println!("----------------------------------------------------------");

	let mut hmap = HashMap::new();
	hmap.insert("hkey1", "value1");
	hmap.insert("hkey2", "value2");
	hmap.insert("hkey3", "value3");
	println!("hmap is {:?}", hmap);
	for x in hmap.iter() {
		println!("hmap: {}->{}", x.0, x.1);
	}

	if let Some(x) = hmap.get("hkey1") {
		println!("hkey1={}", x);
		hmap.remove("hkey1");
		if let Some(x) = hmap.get("hkey1") {
			println!(">>hkey1={}", x);
		} else {
			println!(">>hkey1 no exist!");
		}
	} else {
		println!("hkey1 no exist!");
	}
}

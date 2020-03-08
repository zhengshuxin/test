use std::collections::BTreeMap;
use std::collections::HashMap;
fn main() {
	let mut bmap = BTreeMap::new();
	bmap.insert("key1", "value1");
	bmap.insert("key2", "value2");
	bmap.insert("key3", "value3");
	println!("bmap is {:?}", bmap);
	for x in bmap.iter() {
		println!("bmap: {:?}", x);
	}

	let mut hmap = HashMap::new();
	hmap.insert("key1", "value1");
	hmap.insert("key2", "value2");
	hmap.insert("key3", "value3");
	println!("hmap is {:?}", hmap);
	for x in hmap.iter() {
		println!("hmap: {}->{}", x.0, x.1);
	}
}

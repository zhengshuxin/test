use std::net::*;
use std::io;
use std::io::{Read, Write};
use std::env;
use std::thread;

/*
fn server(addr: ToSocketAddrs) -> io::Result() {
    let listener = try!(TcpListener::bind(&addr));
    for stream in listener.incoming() {
        match stream {
            Ok(mut st) => {
                let mut buf: Vec[u8] = vec![0u8; 1024];
                let rcount = try!(st.read(&mut buf));
                println!("{:?}", &buf[0..rcount]);
                let wcount = try!(st.write(&buf[0..rcount]));
                if (wcount != rcount) {
                    println!("not fully, wcount={}, rcount={}", wcount, rcount);
                    break;
                }
                buf.clear();
            }
            Err(e) => { println!("error={}", e); break; }
        }
    }

    drop(listener);
    Ok(());
}

*/

fn server<A: ToSocketAddrs>(addr: A) -> io::Result<()> {
    // 建立一个监听程序
    let listener = try!(TcpListener::bind(&addr)) ;
    // 这个程序一次只需处理一个链接就好
    for stream in listener.incoming() {
        // 通过match再次解包 stream到
        match stream {
            // 这里匹配的重点是如何将一个mut的匹配传给一个Result
            Ok(mut st) => {
                // 我们总是要求client端先发送数据
                // 准备一个超大的缓冲区
                // 当然了， 在实际的生活中我们一般会采用环形缓冲来重复利用内存。
                // 这里仅作演示， 是一种很低效的做法
                let mut buf: Vec<u8> = vec![0u8; 1024];
                // 通过try!方法来解包
                // try!方法的重点是需要有特定的Error类型与之配合
                let rcount = try!(st.read(&mut buf));
                // 只输出缓冲区里读取到的内容
                println!("{:?}", &buf[0..rcount]);
                // 回写内容
                let wcount = try!(st.write(&buf[0..rcount]));
                // 以下代码实际上算是逻辑处理
                // 并非标准库的一部分了
                if rcount != wcount {
                    panic!("Not Fully Echo!, r={}, w={}", rcount, wcount);
                } /
                / 清除掉已经读到的内容
                    buf.clear();
            } E
            rr(e) => {
                panic!("{}", e);
            }
        }
    } /
    / 关闭掉Serve端的链接
    drop(listener);
    Ok(())
}

fn main() {

}

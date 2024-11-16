// // open in editor 服务
// // const http = require("http");
// // const urlParser = require("url");
// // const path = require("path");
// // const child_process = require("child_process");
// // const chalk = require('chalk');
// import { networkInterfaces } from 'os';
// import http from 'http';
// import { parse } from 'url';
// import path from 'path';
// import { exec } from 'child_process';
// import chalk from 'chalk';

// // 从 package.json 中区端口。fcm 端口 - 1000，fcm-dev-tool 也使用这个规则配置端口
// const port = process.env.npm_package_devPort - 1000 || 3000;
// const hostname = 'localhost';

// const server = http.createServer((req, res) => {
//   const { url } = req;
//   setHeader(res);
//   const urlObj = parse(url, true);
//   const pathname = urlObj.query.pathname;
//   const srcRoot = urlObj.query.srcRoot || 'src';
//   if (!pathname) {
//     res.end('open in editor 服务，需要参数 pathname 参数');
//     return;
//   }
//   exec(`code -r -g  ${path.join(srcRoot + pathname)}`);
//   res.statusCode = 200;
//   res.end('success');
// });

// function setHeader(res) {
//   res.setHeader('Content-Type', 'text/plain;charset=utf8');
//   res.setHeader('Access-Control-Allow-Origin', '*');
//   res.setHeader('Access-Control-Allow-Credentials', true);
//   res.setHeader('Access-Control-Allow-Headers', 'customer-header, customer-header2');
//   res.setHeader('Access-Control-Allow-Methods', 'POST, PUT');
// }
// /**
//  * 获取本地 ip 地址
//  * @returns
//  */
// function getLocalIPAddresses() {
//   const interfaces = networkInterfaces();
//   const addresses = [];
//   for (const name of Object.keys(interfaces)) {
//     for (const iface of interfaces[name]) {
//       if (iface.family === 'IPv4' && !iface.internal) {
//         addresses.push(iface.address);
//       }
//     }
//   }

//   return addresses;
// }

// server.listen(port, () => {
//   const localIPs = getLocalIPAddresses();
//   localIPs.forEach((ip) => {
//     // console.log(`dev-server running on http://${ip}:${port}`);
//     console.log(`front end running on http://${ip}:${port + 1000}`);
//   });
// });

// // # git update-index --assume-unchanged src/static/platform/js/fcm.js
// // # git update-index --no-assume-unchanged src/static/platform/js/fcm.js

// // ctrl + 鼠标中键打开 html

// use std::env;
use std::fs;
use std::path;
use std::io::{Read, Write};
use std::process::Command;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::thread;

fn main() {
    // let port = env::var("npm_package_devPort").unwrap().parse::<u16>().unwrap() - 1000;
    // 从 ./package.json 文件中获取端口。fcm 端口 - 1000
    let port = fs::read_to_string("./package.json")
        .unwrap()
        .lines()
        .find(|line| line.starts_with("\"devPort\": "))
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .parse::<u16>()
        .unwrap() - 1000;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let listener = TcpListener::bind(addr).unwrap();
    println!("dev-server running on http://localhost:{}", port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let request = String::from_utf8_lossy(&buffer);
            let url = request.lines().nth(0).unwrap();
            let url_obj = url.split('?').collect::<Vec<&str>>()[1];
            let url_obj_map = url_obj.split('&').map(|x| x.split('=').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
            let pathname = url_obj_map.iter().find(|x| x[0] == "pathname").unwrap()[1];
            let src_root = url_obj_map.iter().find(|x| x[0] == "srcRoot").unwrap_or(&["src"])[1];
            let mut cmd = Command::new("code");
            cmd.arg("-r");
            cmd.arg("-g");
            cmd.arg(path::Path::new(src_root).join(pathname));
            let output = cmd.output().unwrap();
            println!("{:?}", String::from_utf8_lossy(&output.stdout));
            println!("{:?}", String::from_utf8_lossy(&output.stderr));
            let response = "success";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        });
    }
}


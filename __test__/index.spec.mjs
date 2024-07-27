import test from 'ava';
import fs from "fs";
import { generateGroupAvatar } from '../index.js';
test('generate_group_avatar from native', (t) => {
  let arr = [
    "avatar1.png",
    "avatar1.png",
    "avatar1.png",
    "avatar1.png",
    "avatar1.png",
  ]
  let files =  arr.map((item)=>{
    return fs.readFileSync(item)
  })
  let res = generateGroupAvatar(files, 2)
  console.log(res);
  fs.writeFileSync("js_bf.png",res)
  t.is(1, 1)
})

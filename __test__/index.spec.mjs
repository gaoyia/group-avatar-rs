import test from 'ava';
import fs from "fs";
import { generateGroupAvatar } from '../index.js';
test('buffer to buffer', async (t) => {
  try {
    fs.rmSync("__test__/cache/group_avatar.png")
  } catch (error) {
    
  }

  let arr = [
    "avatar1.png",
    "avatar1.png",
    "avatar1.png",
  ]
  let files =  arr.map((item)=>{
    return fs.readFileSync(item)
  })
  let res;
  try {
    res = await generateGroupAvatar({
      images:files,
      size:600,
      margin: 600/30,
      borderMargin: 600/30,
    })
  } catch (error) {
    console.log(error);
    throw error;
  }
  fs.writeFileSync("__test__/cache/group_avatar.png",res)
  let fileExist = fs.statSync("__test__/cache/group_avatar.png");
  t.is(Boolean(fileExist), true)
})

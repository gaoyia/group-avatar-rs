import test from 'ava';
import fs from "fs";
import { generateGroupAvatar } from '../index.js';
test('buffer to buffer', async (t) => {
  try {
    fs.rmSync("__test__/group_avatar.png")
  } catch (error) {
    
  }

  let arr = [
    "__test__/avatar.png",
    "__test__/avatar.png",
    "__test__/avatar.png",
  ]
  let files =  arr.map((item)=>{
    return fs.readFileSync(item)
  })
  let res;
  try {
    res = await generateGroupAvatar({
      images:files,
      size:600,
      margin: 600 / 20,
      borderMargin: 600 / 20,
      bgColor: [222, 222, 222, 255], // RGBA
    })
  } catch (error) {
    console.log(error);
    throw error;
  }
  fs.writeFileSync("__test__/group_avatar.png",res)
  let fileExist = fs.statSync("__test__/group_avatar.png");
  t.is(Boolean(fileExist), true)
})

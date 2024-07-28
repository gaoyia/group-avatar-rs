# nodejs rs addon

使用rust开发的 nodejs addon

用于高效的创建群组头像

## 使用

```js
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
      images:files, // 图片buffer的数组
      size:600, // 头像的大小
      margin: 600/30, // 头像之间的间距
      borderMargin: 600/30, // 头像与边框之间的间距
    })
  } catch (error) {
    console.log(error);
    throw error;
  }
  fs.writeFileSync("__test__/cache/group_avatar.png",res)

```

```ts
export interface Config {
  images: Array<Buffer>
  size?: number
  borderMargin?: number
  margin?: number
  saveFile?: boolean // 是否保存文件 ,如果你想直接保存文件省去js的保存文件步骤可以将该选项设置为true
  savePath?: string // 保存文件的路径
  bgFile?: string // 背景图片路径，暂不支持设置rgba的背景颜色，后续考虑支持
}
```
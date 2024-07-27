import test from 'ava'

import a from '../index.js'
console.log(a);
test('generate_group_avatar from native', (t) => {
  let arr = [
    "avatar1.png",
    "avatar1.png",
    "avatar1.png",
    "avatar1.png",
    "avatar1.png",
]
  t.is(a.generateGroupAvatar(arr, 2), 1)
})

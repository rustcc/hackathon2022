# 项目介绍
一个类似swagger的api文档实现，旨在增加web开发效率，方便前后端同学联调。前端使用sycamore实现，这样的好处是可以复用后端数据结构，将来可以使用生成的接口描述信息增加调试，自动测试，自动生成模板页面等功能，这对类似后端管理系统样式比较统一的web应用很方便。目前参数字段已适配了serde json的rename,rename_all,skip,flatten等宏参数。
  
1.执行 cargo run --package demo-app <br>
2.浏览器访问 http://127.0.0.1:9001/static/resources/dist/index.html

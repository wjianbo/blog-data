---
title: UiPath DataTable 操作
date: 2020-02-05 09:58:15
tags: [UiPath]
---

## 筛选
使用 Filter Data Table Activity 可以对 DataTable 的数据进行筛选。

如果是通过 DataScraping 获取的 DataTable，因为数据类型都是 String，无法直接比较数值大小。

解决方法：

```
aDataTable.Select("Convert([WIID],'System.Int32')<200000").CopyToDataTable
```

.CopyToDataTable 可以将 .Select 得到的 DataRow[] 复制到新 DataTable。

参考： [Filter Data Table Activity](https://forum.uipath.com/t/filter-data-table-activity/76465) 

## 复制 DataRow

.CopyToDataTable 只适用于 DataRow[]，如果要把单独一条 Datarow 复制到新的 DataTable，可尝试以下方法：

1. use Build Datatabel Activity set output as dtO
2. use Assign activity dtO=yourDatatable.Clone()
3. use Add AddROW Activity.
4.  set ArrayROW in item.ItemArray(), Datatable=dtO

参考：[Copy Row from one DataTable to another - RPA Developers - UiPath Community Forum](https://forum.uipath.com/t/copy-row-from-one-datatable-to-another/80/14)
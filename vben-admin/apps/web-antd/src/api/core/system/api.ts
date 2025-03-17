import type { Pagination } from '..';

import { requestClient } from '#/api/request';

export interface ApiItem {
  id: number;
  description: string;
  method: string;
  path: string;
  group: string;
  api_key: string;
}
export async function getTreeApi() {
  const { items } = await getApis({
    page: 1,
    page_size: 10_000,
    sort_by: 'id',
    sort_order: 'asc',
  });
  // 按照group字段组装成树形结构
  const treeData: any[] = [];
  items.forEach((item) => {
    let index = treeData.findIndex((tree) => tree.title === item.group);
    if (index === -1) {
      treeData.push({
        title: item.group,
        key: 0,
        children: [],
      });
      index = treeData.length - 1;
    }
    treeData[index].children.push({
      ...item,
      key: item.id,
      title: `${item.description}[${item.path}][${item.api_key}]`,
    });
  });
  return treeData;
}
export async function getApis(data: {
  api_key?: string;
  description?: string;
  page: number;
  page_size: number;
  path?: string;
  sort_by: string;
  sort_order: string;
}) {
  const apiList = await requestClient.get<{
    items: ApiItem[];
    pagination: Pagination;
  }>('/system/api/list', {
    params: data,
  });
  return apiList;
}

export async function saveApi(data: ApiItem) {
  return await requestClient.post('/system/api/save', data);
}

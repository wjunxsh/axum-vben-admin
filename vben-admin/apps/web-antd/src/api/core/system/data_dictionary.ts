import type { Pagination } from '..';

import { requestClient } from '#/api/request';

export interface DataDictionaryItem {
  id: number;
  name: string;
  status: string;
  remark: string;
  code: string;
}
export async function getDataDictionaries(data: {
  page: number;
  page_size: number;
  sort_by: string;
  sort_order: string;
}) {
  const dataDictonaryList = await requestClient.get<{
    items: DataDictionaryItem[];
    pagination: Pagination;
  }>('/system/dictionary/list', {
    params: data,
  });
  return dataDictonaryList;
}

export async function saveDataDictonary(data: DataDictionaryItem) {
  return await requestClient.post('/system/dictionary/save', data);
}

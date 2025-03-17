import type { Pagination } from '..';

import { requestClient } from '#/api/request';

export interface DataDictionaryConfigItem {
  id: number;
  label: string;
  value: string;
  sort: number;
  status: string;
  remark: string;
  data_dictionary_id: number;
}
export async function getDataDictionaryConfigs(data: {
  data_dictionary_id: number;
  page: number;
  page_size: number;
  sort_by: string;
  sort_order: string;
}) {
  const dataDictonaryList = await requestClient.get<{
    items: DataDictionaryConfigItem[];
    pagination: Pagination;
  }>('/system/dictionary/config_list', {
    params: data,
  });
  return dataDictonaryList;
}
export const deleteDataDictionaryConfig = async (id: number) => {
  return await requestClient.delete(`/system/dictionary/delete_config/${id}`);
};

export async function saveDataDictonaryConfig(data: DataDictionaryConfigItem) {
  return await requestClient.post('/system/dictionary/save_config', data);
}

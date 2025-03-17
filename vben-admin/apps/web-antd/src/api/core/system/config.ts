import type { Pagination } from '..';

import { requestClient } from '#/api/request';

export interface ConfigItem {
  id: number;
  name: string;
  key: string;
  value: string;
  remark: string;
}
export async function getConfigs(data: {
  api_key?: string;
  description?: string;
  page: number;
  page_size: number;
  path?: string;
  sort_by: string;
  sort_order: string;
}) {
  const apiList = await requestClient.get<{
    items: ConfigItem[];
    pagination: Pagination;
  }>('/system/config/list', {
    params: data,
  });
  return apiList;
}

export async function saveConfig(data: ConfigItem) {
  return await requestClient.post('/system/config/save', data);
}

import type { Pagination } from '..';

import { requestClient } from '#/api/request';

export interface AdminLogItem {
  id: number;
  path: string;
  operate_name: string;
  created_at: number;
  system_real_name: string;
  method: string;
  ip: string;
  user_agent: string;
  response_time_ms: number;
  response_status: number;
  error_message: string;
}

export async function getLogs(data: {
  api_key?: string;
  description?: string;
  page: number;
  page_size: number;
  path?: string;
  sort_by: string;
  sort_order: string;
}) {
  const logList = await requestClient.get<{
    items: AdminLogItem[];
    pagination: Pagination;
  }>('/system/log/list', {
    params: data,
  });
  return logList;
}

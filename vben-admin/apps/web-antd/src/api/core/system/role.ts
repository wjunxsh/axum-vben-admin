import type { Pagination } from '..';

import { requestClient } from '#/api/request';

export interface RoleItem {
  id: number;
  role_name: string;
  status: string;
}

export async function getSystemRoles(data: {
  page: number;
  pageSize: number;
  sort_by: string;
  sort_order: string;
}) {
  const userList = await requestClient.get<{
    items: RoleItem[];
    pagination: Pagination;
  }>('/system/role/list', {
    params: data,
  });
  return userList;
}
export async function getSystemRoleMenus(role_id: number) {
  const role_menus_ids = await requestClient.get<number[]>(
    '/system/role/menu_list',
    {
      params: { role_id },
    },
  );
  return role_menus_ids;
}
export async function saveSystemRoleMenus(role_id: number, menu_ids: number[]) {
  return requestClient.post<number[]>('/system/role/save_menu', {
    system_role_id: role_id,
    system_menu_ids: menu_ids,
  });
}
export async function getAccessApiIds(role_id: number) {
  const role_api_ids = await requestClient.get<number[]>(
    '/system/role/api_list',
    {
      params: { role_id },
    },
  );
  return role_api_ids;
}
export async function saveSystemRoleApis(role_id: number, api_ids: number[]) {
  return requestClient.post<number[]>('/system/role/save_api', {
    system_role_id: role_id,
    system_api_ids: api_ids,
  });
}
export async function saveRole(data: RoleItem) {
  return await requestClient.post('/system/role/save', data);
}
export async function saveRoleMenuAccess(data: {
  menu_ids: number[];
  role_id: number;
}) {
  return await requestClient.post('/system/role/save_menu', data);
}

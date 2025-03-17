import type { Pagination } from '..';

import { requestClient } from '#/api/request';

export interface UserRole {
  id: number;
  role_name: string;
}
export interface UserItem {
  id: number;
  /**
   * accessToken
   */
  token?: string;
  homePath?: string;
  /**
   * 真实姓名
   */
  real_name: string;
  password: string;
  /**
   * 登录名称
   */
  user_name: string;
  /**
   * 状态
   */
  status: string;
  /**
   * 头像
   */
  avatar: string;
  /**
   * 是否管理员
   */
  is_admin: 0 | 1;
  roles?: UserRole[];
}

/**
 * 获取用户信息
 */
export async function getUserInfoApi() {
  return requestClient.get<UserItem>('/system/user/info');
}

export async function getSystemUsers(data: {
  page: number;
  pageSize: number;
  sort_by: string;
  sort_order: string;
}) {
  const userList = await requestClient.get<{
    items: { user: UserItem }[];
    pagination: Pagination;
  }>('/system/user/list', {
    params: data,
  });
  return userList;
}
export async function getAccessUserInfoApi() {
  return requestClient.get<UserItem>('/system/access/user_info');
}
export async function saveUser(data: { role_ids: number[]; user: UserItem }) {
  return await requestClient.post('/system/user/save', data);
}

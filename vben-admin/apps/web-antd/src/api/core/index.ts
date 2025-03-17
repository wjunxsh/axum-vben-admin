export * from './system/api';
export * from './system/auth';
export * from './system/menu';
export * from './system/role';
export * from './system/user';
export interface Pagination {
  page: number;
  page_size: number;
  total: number;
}

import type { RouteRecordStringComponent } from '@vben/types';

import { requestClient } from '#/api/request';

export interface Meta {
  title: string;
  icon: string;
  activeIcon?: string;
  keepAlive?: boolean;
  hideInMenu?: boolean;
  hideInTab?: boolean;
  hideInBreadcrumb?: boolean;
  authority?: string[];
  affixTab?: boolean;
  link?: string;
  openInNewWindow?: boolean;
  order?: number;
}
export interface MenuItem {
  id: number;
  name: string;
  title: string;
  path: string;
  key?: number;
  link: string;
  parent_id: number;
  component: string;
  children: MenuItem[];
  meta: Meta;
}

const getRouteRecordStringComponent = (
  menuItem: MenuItem[],
  parentPath = '',
): RouteRecordStringComponent[] => {
  if (menuItem && menuItem.length === 0) {
    return [];
  }
  const routeRecordStringComponent: RouteRecordStringComponent[] = [];
  for (const item of menuItem) {
    const route: RouteRecordStringComponent = {
      path: parentPath + item.path,
      name: item.name,
      component: item.component,
      meta: {
        icon: item.meta.icon,
        title: item.meta.title,
        link: item.link,
        hideInMenu: item.meta.hideInMenu,
        keepAlive: item.meta.keepAlive,
        hideInTab: item.meta.hideInTab,
        affixTab: item.meta.affixTab,
      },
      children: getRouteRecordStringComponent(item.children, item.path),
    };
    routeRecordStringComponent.push(route);
  }
  return routeRecordStringComponent;
};

const getMenuTree = (menuList: MenuItem[]): MenuItem[] => {
  const menus: MenuItem[] = [];
  for (const menuListItem of menuList) {
    const menu: MenuItem = {
      id: menuListItem.id,
      title: menuListItem.title,
      link: menuListItem.link,
      name: menuListItem.name,
      key: menuListItem.id,
      path: menuListItem.path,
      component: menuListItem.component,
      parent_id: menuListItem.parent_id,
      children: [],
      meta: {
        hideInMenu: menuListItem.meta.hideInMenu,
        hideInTab: menuListItem.meta.hideInTab,
        affixTab: menuListItem.meta.affixTab,
        hideInBreadcrumb: menuListItem.meta.hideInBreadcrumb,
        openInNewWindow: menuListItem.meta.openInNewWindow,
        icon: menuListItem.meta.icon,
        link: menuListItem.link ?? '',
        title: menuListItem.title ?? '',
      },
    };
    menus.push(menu);
  }
  for (const menuItem of menus) {
    const index = menus.findIndex((item) => item.id === menuItem.parent_id);
    if (index !== -1) {
      menus[index]?.children.push(menuItem);
    }
  }
  return menus.filter((item) => item.parent_id === 0);
};

/**
 * 获取用户所有菜单
 */
export async function getAllMenusApi() {
  const menuList = await requestClient.get<MenuItem[]>(
    '/system/menu/access_list',
  );
  const menus = getMenuTree(menuList);
  return getRouteRecordStringComponent(menus);
}

export async function getMenusApi(data: {
  page: number;
  pageSize: number;
  sortBy: string;
  sortOrder: string;
}) {
  const menuList = await requestClient.get<MenuItem[]>('/system/menu/list', {
    params: data,
  });
  return menuList;
}

export async function getMenusTreeApi() {
  const menuList: MenuItem[] = await getMenusApi({
    page: 1,
    pageSize: 100,
    sortBy: 'id',
    sortOrder: 'asc',
  });
  return getMenuTree([...menuList]);
}

export async function addMenuApi(data: MenuItem) {
  return await requestClient.post('/system/menu/save', data);
}

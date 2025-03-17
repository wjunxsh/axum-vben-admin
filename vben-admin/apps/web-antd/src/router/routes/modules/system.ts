import type { RouteRecordRaw } from 'vue-router';

import { $t } from '#/locales';

const routes: RouteRecordRaw[] = [
  {
    meta: {
      icon: 'lucide:layout-dashboard',
      order: 1,
      title: $t('page.system.title'),
    },
    name: 'System',
    path: '/system',
    children: [],
  },
];

export default routes;

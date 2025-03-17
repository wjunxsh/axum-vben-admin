<script lang="ts" setup>
import { ref } from 'vue';

import { useVbenDrawer } from '@vben/common-ui';

import { message, TabPane, Tabs, Tree } from 'ant-design-vue';

import {
  getAccessApiIds,
  getMenusTreeApi,
  getSystemRoleMenus,
  getTreeApi,
  saveSystemRoleApis,
  saveSystemRoleMenus,
} from '#/api';

const emit = defineEmits<{
  refresh: [];
}>();
const treeData = ref<any>([]);
const expandedKeys = ref<number[]>([]);
const checkedKeys = ref<number[]>([]);
const apiTreeData = ref<any>([]);
const apiExpandedKeys = ref<number[]>([]);
const apiCheckedKeys = ref<number[]>([]);
const [Drawer, drawerApi] = useVbenDrawer({
  onCancel() {
    drawerApi.close();
  },
  onClosed() {
    drawerApi.setState({ overlayBlur: 0, placement: 'right' });
  },
  async onOpenChange(open) {
    if (open) {
      const data = drawerApi.getData();
      const menuTree = await getMenusTreeApi();
      const role_menus_ids = await getSystemRoleMenus(data.id);
      treeData.value = menuTree;
      expandedKeys.value = treeData.value
        .filter((item: any) => item.parent_id === 0)
        .map((item: any) => item.key);
      checkedKeys.value = role_menus_ids;
      const apiTree = await getTreeApi();
      const role_api_ids = await getAccessApiIds(data.id);
      apiTreeData.value = apiTree;
      apiCheckedKeys.value = role_api_ids;
      apiExpandedKeys.value = apiTreeData.value.map((item: any) => item.key);
    }
  },
  async onConfirm() {
    message.info('onConfirm');
    await saveSystemRoleMenus(drawerApi.getData().id, checkedKeys.value);
    await saveSystemRoleApis(
      drawerApi.getData().id,
      apiCheckedKeys.value.filter((item) => item > 0),
    );
    drawerApi.close();
    emit('refresh');
  },
});
</script>
<template>
  <Drawer title="分配权限" title-tooltip="">
    <Tabs>
      <TabPane key="1" tab="菜单">
        <Tree
          v-model:checked-keys="checkedKeys"
          v-model:expanded-keys="expandedKeys"
          checkable
          :tree-data="treeData"
        />
      </TabPane>
      <TabPane key="2" tab="接口">
        <Tree
          v-model:checked-keys="apiCheckedKeys"
          v-model:expanded-keys="apiExpandedKeys"
          checkable
          :tree-data="apiTreeData"
        />
      </TabPane>
    </Tabs>
  </Drawer>
</template>

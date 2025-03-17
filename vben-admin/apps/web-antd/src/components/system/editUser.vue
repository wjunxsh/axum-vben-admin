<script lang="ts" setup>
import { useVbenModal, z } from '@vben/common-ui';

import md5 from 'md5';

import { useVbenForm } from '#/adapter/form';
import { getSystemRoles, saveUser } from '#/api';

const emit = defineEmits<{
  refresh: [];
}>();
const [CustomLayoutForm, formApi] = useVbenForm({
  // 所有表单项共用，可单独在表单内覆盖
  commonConfig: {
    // 所有表单项
    componentProps: {
      class: 'w-full',
    },
  },

  layout: 'horizontal',
  schema: [
    {
      component: 'InputNumber',
      fieldName: 'id',
      defaultValue: 0,
      label: 'ID',
      formItemClass: 'col-span-6',
      dependencies: {
        show: false,
        triggerFields: ['id'],
      },
    },
    {
      component: 'Input',
      fieldName: 'user_name',
      label: '用户名称',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'real_name',
      label: '真实用户名',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'password',
      label: '密码',
      formItemClass: 'col-span-6',
      dependencies: {
        rules(values) {
          if (!values.id) {
            return z.string().min(6, '密码长度不能小于6位');
          }
          return null;
        },
        triggerFields: ['id'],
      },
      // rules: z.string().min(6, '密码长度不能小于6位'),
    },
    {
      component: 'RadioGroup',
      formItemClass: 'col-span-6',
      componentProps: {
        options: [
          {
            label: '在职',
            value: 'valid',
          },
          {
            label: '离职',
            value: 'invalid',
          },
        ],
      },
      fieldName: 'status',
      label: '状态',
    },
    {
      component: 'Checkbox',
      formItemClass: 'col-span-6',
      fieldName: 'is_adminis_admin',
      label: '',
      renderComponentContent: () => {
        return {
          default: () => ['系统管理员'],
        };
      },
    },
    {
      component: 'ApiSelect',
      fieldName: 'role_ids',
      formItemClass: 'col-span-3',
      // 对应组件的参数
      componentProps: {
        mode: 'multiple',
        allowClear: true,
        // 菜单接口
        api: async () => {
          const { items } = await getSystemRoles({
            page: 1,
            pageSize: 1000,
            sort_by: '',
            sort_order: '',
          });
          return items;
        },
        // 菜单接口转options格式
        labelField: 'role_name',
        valueField: 'id',
        childrenField: 'children',
      },
      // 界面显示的label
      label: '用户角色',
    },
  ],
  // 一共三列
  wrapperClass: 'grid-cols-6',
  showDefaultActions: false,
});
const [Modal, modalApi] = useVbenModal({
  onCancel() {
    modalApi.close();
  },
  async onConfirm() {
    const result = await formApi.validateAndSubmitForm();
    if (!result) {
      return;
    }
    const formData = await formApi.getValues();
    let password = '';
    if (formData.password) {
      password = md5(formData.password);
    }
    await saveUser({
      user: {
        id: formData.id ?? 0,
        user_name: formData.user_name,
        real_name: formData.real_name,
        avatar: formData.avatar ?? '',
        password,
        is_admin: formData.is_admin ? 1 : 0,
        status: formData.status,
      },
      role_ids: formData.role_ids,
    });
    // message.info('onConfirm');
    modalApi.close();
    emit('refresh');
  },
  onOpenChange(isOpen: boolean) {
    if (isOpen) {
      const { values } = modalApi.getData<Record<string, any>>();
      if (values) {
        formApi.setValues(values);
      }
    }
  },
  onOpened() {
    // message.info('onOpened：打开动画结束');
  },
});
</script>

<template>
  <Modal draggable class="w-[800px]" title="编辑菜单" title-tooltip="修改菜单">
    <CustomLayoutForm />
  </Modal>
</template>

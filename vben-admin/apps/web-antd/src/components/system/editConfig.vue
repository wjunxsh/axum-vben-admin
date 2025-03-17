<script lang="ts" setup>
import { useVbenModal } from '@vben/common-ui';

import { useVbenForm } from '#/adapter/form';
import { saveConfig } from '#/api/core/system/config';

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
      fieldName: 'name',
      label: '参数名',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'key',
      label: '参数键',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'Textarea',
      fieldName: 'value',
      label: '参数值',
      formItemClass: 'col-span-6',
      // rules: z.string().min(6, '密码长度不能小于6位'),
    },
    {
      component: 'Textarea',
      fieldName: 'remark',
      label: '备注',
      formItemClass: 'col-span-6',
      // rules: z.string().min(6, '密码长度不能小于6位'),
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
    await saveConfig({
      id: formData.id ?? 0,
      name: formData.name,
      remark: formData.remark,
      key: formData.key,
      value: formData.value,
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
  <Modal draggable class="w-[800px]" title="编辑配置" title-tooltip="修改配置">
    <CustomLayoutForm />
  </Modal>
</template>

<script lang="ts" setup>
import { useVbenModal } from '@vben/common-ui';

import { useVbenForm } from '#/adapter/form';
import { saveDataDictonaryConfig } from '#/api/core/system/data_dictionary_config';

const emit = defineEmits<{
  refresh: [];
}>();
const [DataDictionaryConfigForm, form] = useVbenForm({
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
      fieldName: 'label',
      label: '配置项名',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'Input',
      fieldName: 'value',
      label: '配置项值',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'InputNumber',
      fieldName: 'sort',
      label: '排序',
      formItemClass: 'col-span-6',
      rules: 'required',
    },
    {
      component: 'RadioGroup',
      formItemClass: 'col-span-6',
      componentProps: {
        options: [
          {
            label: '启用',
            value: 1,
          },
          {
            label: '停用',
            value: 0,
          },
        ],
      },
      fieldName: 'status',
      label: '状态',
    },
    {
      component: 'Input',
      fieldName: 'remark',
      label: '备注',
      formItemClass: 'col-span-6',
      rules: 'required',
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
    const result = await form.validateAndSubmitForm();
    if (!result) {
      return;
    }
    const formData = await form.getValues();
    await saveDataDictonaryConfig({
      id: formData.id ?? 0,
      data_dictionary_id:
        modalApi.getData<Record<string, any>>().data_dictionary_id,
      label: formData.label,
      remark: formData.remark,
      value: formData.value,
      status: formData.status,
      sort: formData.sort,
    });
    // message.info('onConfirm');
    modalApi.close();
    emit('refresh');
  },
  onOpenChange(isOpen: boolean) {
    if (isOpen) {
      const { values } = modalApi.getData<Record<string, any>>();
      if (values) {
        form.setValues(values);
      }
    }
  },
  onOpened() {
    // message.info('onOpened：打开动画结束');
  },
});
</script>

<template>
  <Modal
    draggable
    class="w-[800px]"
    title="编辑配置项"
    title-tooltip="编辑配置项"
  >
    <DataDictionaryConfigForm />
  </Modal>
</template>

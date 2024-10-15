import { Button, Checkbox, Form, Input, Spin } from "antd";
import { forwardRef, useEffect, useImperativeHandle } from "react";
import { v4 } from "uuid";

interface CronFormProps {
  loading: boolean;
  job?: CronJob;
  onSubmit?: (job: CronJob) => void;
}
export type CronJob = {
  id: string;
  command: string;
  cron: string;
  comment: string;
  _disabled: boolean;
};
export default forwardRef(function CronForm(props: CronFormProps, ref) {
  const [form] = Form.useForm();
  useImperativeHandle(ref, () => {
    return {
      resetFields: form.resetFields,
      setFieldsValue: form.setFieldsValue
    };
  })
  
  useEffect(() => {
    form.setFieldsValue(props.job || {});
  }, [props.job]);

  const onSubmit = async () => {
    try {
      let values = await form.validateFields();
      debugger
      if (props.job?.id) {
        values.id = props.job.id;
      }
      if (!values.id) values.id = v4();
      props.onSubmit?.(values);
    } catch (errorInfo) {
      console.log("Failed:", errorInfo);
    }
  };
  return (
    <div>
      <Spin spinning={props.loading}>
        <Form
          form={form}
          name="basic"
          labelCol={{ span: 8 }}
          wrapperCol={{ span: 16 }}
          style={{ maxWidth: 600 }}
          autoComplete="off"
        >
          <Form.Item<CronJob> label="Name / Comment" name="comment">
            <Input />
          </Form.Item>

          <Form.Item<CronJob>
            label="Command"
            name="command"
            rules={[{ required: true, message: "Please input your command!" }]}
          >
            <Input />
          </Form.Item>

          <Form.Item<CronJob>
            label="Cron"
            name="cron"
            rules={[{ required: true, message: "Please input your cron!" }]}
          >
            <Input />
          </Form.Item>

          <Form.Item<CronJob>
            label="Disabled"
            name="_disabled"
            valuePropName="checked"
          >
            <Checkbox>禁用</Checkbox>
          </Form.Item>

          <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
            <Button type="primary" onClick={onSubmit}>
              Submit
            </Button>
          </Form.Item>
        </Form>
      </Spin>
    </div>
  );
});
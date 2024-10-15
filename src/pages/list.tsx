import CronSider from "@/components/cron-sider";
import CronForm, { CronJob } from "@/components/cron-form";
import { invoke } from "@tauri-apps/api/core";
import { useMount } from "ahooks";
import { useRef, useState } from "react";
import { Layout, message } from "antd";
import { v4 } from "uuid";

const { Sider, Content } = Layout;

export default function ListPage() {
  const [loading, setLoading] = useState(false);
  const [jobs, setJobs] = useState<CronJob[]>([]);
  const [job, setJob] = useState<CronJob | undefined>();
  const cronFormRef = useRef<{
    resetFields: () => void;
  }>(null);
  useMount(() => {
    fetchCrontabJobs();
  });

  async function fetchCrontabJobs() {
    setLoading(true);
    try {
      setLoading(true);
      const jobs = await invoke<any[]>("load_crons");
      setLoading(false);
     
      const jobsWithId = jobs.map((job) => {
        return {
          ...job,
          id: v4(),
        };
      })
      console.log("Crontab Jobs:", jobsWithId);
      setJobs(jobsWithId);
    } catch (error) {
      console.error("Error fetching crontab jobs:", error);
    }
  }
  async function onSubmit(job: CronJob) {
    const jobIdx = jobs.findIndex((j) => j.id === job.id);
    const newJobs = jobs.map((_job, idx) => {
      if (idx === jobIdx) {
        return job;
      }
      return {
        ..._job,
      };
    });
    try {
      await invoke("save_crons", { jobs: newJobs });
      message.success("保存成功");
      setJob(undefined)
      fetchCrontabJobs();
    } catch (error) {
      console.error("Error adding cron job:", error);
    }
  }
  return (
    <Layout style={{ minHeight: "100vh" }}>
      <Sider theme="light">
        <CronSider
          items={jobs.map((job) => {
            return {
              ...job,
              key: job.id,
              label: job.comment || job.cron,
              cron: job.cron,
              command: job.command,
              _disabled: job._disabled,
            };
          })}
          loading={loading}
          onClick={(job) => {
            setJob(job);
          }}
        />
      </Sider>
      <Layout>
        <Content
          style={{
            margin: "24px 16px",
            padding: 24,
            minHeight: 280,
          }}
        >
          <CronForm loading={loading} job={job} onSubmit={onSubmit} ref={cronFormRef} />
        </Content>
      </Layout>
    </Layout>
  );
}

import { CronJob } from 'cron';
import 'dotenv/config';

const mainExpression = '*/14 * * * *';
const twoSecs = '*/10 * * * * *';

const job = new CronJob(mainExpression, async function () {
  const req = await fetch(process.env.JOLLOF_HEALTH_CHECK);
  const res = await req.json();

  console.log(res);
  console.log('Ya!');
});

job.start();

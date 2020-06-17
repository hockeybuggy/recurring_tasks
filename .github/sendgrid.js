#! /usr/bin/env node

const fs = require('fs');

const sgMail = require('@sendgrid/mail');
sgMail.setApiKey(process.env.SENDGRID_API_KEY);

const subjectBuffer = fs.readFileSync('subject.txt');
const bodyBuffer = fs.readFileSync('body.txt');

const now = new Date(Date.now());
const msg = {
  to: 'hockeybuggy@gmail.com',
  from: 'hockeybuggy+recurring@gmail.com',
  subject: subjectBuffer.toString(),
  text: bodyBuffer.toString(),
};

sgMail
  .send(msg)
  .then(() => console.log('Mail sent successfully'))
  .catch(error => console.error(error.toString()));

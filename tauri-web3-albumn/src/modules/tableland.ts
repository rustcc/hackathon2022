import type { UserModule } from "~/types";

// import { connect } from "@tableland/sdk";
// import { SUPPORTED_CHAINS } from "@tableland/sdk";
// const polygonTestnet = SUPPORTED_CHAINS['polygon-mumbai']

// { app, router, routes, isClient, initialState }
export const install: UserModule = async ({ isClient, app }) => {
  if (!isClient) return;

  // const tableland = await connect({network: 'testnet', chain: 'polygon-mumbai'});
  // app.provide("tableland", tableland);

  // const { name } = await tableland.create(
  //   `name text, id int, primary key (id)`,
  //   `quickstart`
  // );
  // // Wait for the table to be created, then query
  // const writeRes = await tableland.write(
  //   `INSERT INTO ${name} VALUES (0, 'Bobby Tables');`
  // );
  // const readRes = await tableland.read(`SELECT * FROM ${name}`);
  // console.log("====> name, writeRes, readRes :", name, writeRes, readRes);
};

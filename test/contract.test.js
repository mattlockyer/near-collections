const test = require('ava');
const {
	getAccount, init,
	contractAccount,
	recordStart, recordStop,
} = require('./test-utils');
const getConfig = require("../utils/config");
const {
	contractId,
	gas,
	attachedDeposit,
} = getConfig();

// test.beforeEach((t) => {
// });

test('contract is deployed', async (t) => {
	try {
		await contractAccount.functionCall({
			contractId,
			methodName: 'new',
			gas,
		})
	} catch(e) {
		if (!/already been initialized/.test(e)) {
			throw e
		}
	}
	t.true(true)
});

test('test', async (t) => {
	
	await recordStart(contractId)

	await contractAccount.functionCall({
		contractId,
		methodName: 'set',
		args: {
			val: 'a',
		},
		gas,
	})

	await recordStop(contractId)

	const res = await contractAccount.viewFunction(
		contractId,
		'get_key',
		{
			val: 'a',
		},
	)

	console.log(res)


	t.true(true)
});

test('test 2', async (t) => {
	
	await recordStart(contractId)

	const key = await contractAccount.viewFunction(
		contractId,
		'get_key',
		{
			val: 'a',
		},
	)

	await contractAccount.functionCall({
		contractId,
		methodName: 'update',
		args: {
			key,
			val: 'b',
		},
		gas,
	})

	await recordStop(contractId)

	const res = await contractAccount.viewFunction(
		contractId,
		'get_key',
		{
			val: 'b',
		},
	)

	console.log(res)


	t.true(true)
});

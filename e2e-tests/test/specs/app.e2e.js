describe('tankpipe app e2e', () => {
  const selectors = {
    heading: '.form-heading',
    menuButton: '.menu-left .og-button',
    createAccountButton: 'button[title="Create a new account"]',
    accountCard: '.accounts .card',
    accountNameInput: '#name',
    accountOpeningInput: '#startingBalance',
    accountTypeSelect: 'select[name="itemSelect"]',
    accountSaveButton: '.form .widget.buttons .og-button:last-child',
    accountErrorMessage: '.error-msg',
    closeButton: 'button[title="Close"]',
    addTransactionButton: 'button[title="Add a transaction"]',
    scheduleHeading: '.form-heading',
    createScheduleButton: 'button[title="Create a new schedule"]',
    modifierHeading: '.form-heading',
    createModifierButton: 'button[title="Create a new modifier"]',
    netAssetsHeading: '.bs-form-heading',
    netAssetsTable: 'table[aria-label="Net Assets"]',
  };

  async function waitForAccountsUiToLoad() {
    await browser.waitUntil(
      async () => (await $$(selectors.menuButton)).length > 0,
      {
        timeout: 15000,
        timeoutMsg: 'expected the app menu to load',
      }
    );

    const closeButtons = await $$(selectors.closeButton);
    if (closeButtons.length > 0 && (await closeButtons[0].isDisplayed())) {
      await closeButtons[0].click();
    }

    const createAccountButtons = await $$(selectors.createAccountButton);
    const createAccountButton =
      createAccountButtons.length > 0 ? createAccountButtons[0] : null;

    if (!createAccountButton || !(await createAccountButton.isDisplayed())) {
      await navigateToMenu('Accounts');
    }

    await browser.waitUntil(
      async () => {
        const buttons = await $$(selectors.createAccountButton);
        return buttons.length > 0 && (await buttons[0].isDisplayed());
      },
      {
        timeout: 15000,
        timeoutMsg: 'expected accounts actions to load',
      }
    );

    const heading = await $(selectors.heading);
    const refreshedCreateAccountButton = await $(selectors.createAccountButton);
    await heading.waitForDisplayed({ timeout: 15000 });
    await refreshedCreateAccountButton.waitForDisplayed({ timeout: 15000 });

    await browser.waitUntil(
      async () => (await heading.getText()) === 'Accounts',
      {
        timeout: 15000,
        timeoutMsg: 'expected accounts view to open',
      }
    );

    return { heading, createAccountButton: refreshedCreateAccountButton };
  }

  async function openNewAccountForm() {
    const { createAccountButton } = await waitForAccountsUiToLoad();
    await createAccountButton.click();

    const heading = await $(selectors.heading);
    await browser.waitUntil(
      async () => (await heading.getText()) === 'New Account',
      {
        timeout: 15000,
        timeoutMsg: 'expected new account form to open',
      }
    );
  }

  async function navigateToMenu(label) {
    const menuButtons = await $$(selectors.menuButton);
    for (const button of menuButtons) {
      if ((await button.getText()) === label) {
        await button.waitForDisplayed({ timeout: 15000 });
        await browser.execute((element) => {
          element.click();
        }, button);
        return;
      }
    }

    throw new Error(`Menu button "${label}" not found`);
  }

  it('shows core UI scaffolding on first load (happy path)', async () => {
    const { heading, createAccountButton } = await waitForAccountsUiToLoad();

    const headingText = await heading.getText();
    expect(headingText).toBe('Accounts');

    await expect(createAccountButton).toBeDisplayed();

    const accountCard = await $(selectors.accountCard);
    await accountCard.waitForDisplayed({ timeout: 15000 });
  });

  it('shows validation errors when adding an invalid account (validation/error state)', async () => {
    await openNewAccountForm();

    const accountNameInput = await $(selectors.accountNameInput);
    await accountNameInput.waitForDisplayed({ timeout: 15000 });
    await accountNameInput.clearValue();

    const accountOpeningInput = await $(selectors.accountOpeningInput);
    await accountOpeningInput.waitForDisplayed({ timeout: 15000 });
    await accountOpeningInput.setValue('abc');

    const accountTypeSelect = await $(selectors.accountTypeSelect);
    await accountTypeSelect.waitForDisplayed({ timeout: 15000 });
    await accountTypeSelect.selectByVisibleText('Expense');

    const saveAccountButton = await $(selectors.accountSaveButton);
    await saveAccountButton.waitForDisplayed({ timeout: 15000 });
    await saveAccountButton.click();

    const errorMessage = await $(selectors.accountErrorMessage);
    await errorMessage.waitForDisplayed({ timeout: 15000 });

    const errorMessages = await $$(selectors.accountErrorMessage);
    const texts = [];
    for (const message of errorMessages) {
      texts.push(await message.getText());
    }
    const text = texts.join(' ').toLowerCase();
    expect(text).toContain('valid starting balance');
  });

  it('navigates to transactions and shows transaction actions', async () => {
    await waitForAccountsUiToLoad();
    await navigateToMenu('Transactions');

    const addTransactionButton = await $(selectors.addTransactionButton);
    await addTransactionButton.waitForDisplayed({ timeout: 15000 });
  });

  it('navigates to schedules and shows schedule controls', async () => {
    await waitForAccountsUiToLoad();
    await navigateToMenu('Schedules');

    const heading = await $(selectors.scheduleHeading);
    await browser.waitUntil(
      async () => (await heading.getText()) === 'Schedules',
      {
        timeout: 15000,
        timeoutMsg: 'expected schedules view to open',
      }
    );

    const createScheduleButton = await $(selectors.createScheduleButton);
    await createScheduleButton.waitForDisplayed({ timeout: 15000 });
  });

  it('navigates to modifiers and shows modifier controls', async () => {
    await waitForAccountsUiToLoad();
    await navigateToMenu('Modifiers');

    const heading = await $(selectors.modifierHeading);
    await browser.waitUntil(
      async () => (await heading.getText()) === 'Modifiers',
      {
        timeout: 15000,
        timeoutMsg: 'expected modifiers view to open',
      }
    );

    const createModifierButton = await $(selectors.createModifierButton);
    await createModifierButton.waitForDisplayed({ timeout: 15000 });
  });

  it('navigates to net assets and shows the balance sheet table', async () => {
    await waitForAccountsUiToLoad();
    await navigateToMenu('Net Assets');

    const heading = await $(selectors.netAssetsHeading);
    await heading.waitForDisplayed({ timeout: 15000 });
    expect(await heading.getText()).toBe('Net Assets');

    const table = await $(selectors.netAssetsTable);
    await table.waitForDisplayed({ timeout: 15000 });
  });

  it('should be easy on the eyes', async () => {
    const body = await $('body');
    const backgroundColor = await body.getCSSProperty('background-color');
    expect(luma(backgroundColor.parsed.hex)).toBeLessThan(100);
  });

  function luma(hex) {
    if (hex.startsWith('#')) {
      hex = hex.substring(1);
    }

    const rgb = parseInt(hex, 16);
    const r = (rgb >> 16) & 0xff;
    const g = (rgb >> 8) & 0xff;
    const b = (rgb >> 0) & 0xff;
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  }
});

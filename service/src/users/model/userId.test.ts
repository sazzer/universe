import { Some } from '@hqoss/monads';
import { parseUserID } from './userId';

describe('UserID', () => {
  describe('parseUserID', () => {
    test.each<[string, string]>([
      ['052DA6F3-2FE9-41BA-8BED-470529224353', '052da6f3-2fe9-41ba-8bed-470529224353'],
      ['3ded431f-1c60-4d50-a948-41cb3ef28520', '3ded431f-1c60-4d50-a948-41cb3ef28520'],
      ['  2b708924-3c26-40b1-bd89-3ac7b660988d', '2b708924-3c26-40b1-bd89-3ac7b660988d'],
      ['2b708924-3c26-40b1-bd89-3ac7b660988d  ', '2b708924-3c26-40b1-bd89-3ac7b660988d'],
      ['  2b708924-3c26-40b1-bd89-3ac7b660988d  ', '2b708924-3c26-40b1-bd89-3ac7b660988d']
    ])('Parsing a valid User ID: "%s" => "%s"', (input, expected) => {
      const userId = parseUserID(input);
      expect(userId.isSome()).toBe(true);
      expect(userId.unwrap()).toEqual(expected);
    });

    test.each<string>([
      '',
      '  ',
      'invalid',
      '3ded431f-1c60-4d50-a948-41cb3ef2852',
      '3ded431f-1c60-4d50-a948-41cb3ef285200',
      '3ded431f-1c60-4d50-a948-41cb3ef2852o'
    ])('Parsing an invalid User ID: "%s"', (input) => {
      const userId = parseUserID(input);
      expect(userId.isNone()).toBe(true);
    });
  });
});

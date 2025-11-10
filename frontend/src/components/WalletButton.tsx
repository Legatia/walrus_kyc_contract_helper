import { ConnectButton, useCurrentAccount } from '@mysten/dapp-kit';
import { Wallet } from 'lucide-react';
import { shortenAddress } from '@/lib/utils';

export function WalletButton() {
  const account = useCurrentAccount();

  return (
    <div className="flex items-center gap-2">
      {account ? (
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2 px-3 py-2 bg-green-50 border border-green-200 rounded-lg">
            <div className="h-2 w-2 rounded-full bg-green-500"></div>
            <Wallet className="h-4 w-4 text-green-700" />
            <span className="text-sm font-medium text-green-900">
              {shortenAddress(account.address)}
            </span>
          </div>
          <ConnectButton />
        </div>
      ) : (
        <ConnectButton />
      )}
    </div>
  );
}

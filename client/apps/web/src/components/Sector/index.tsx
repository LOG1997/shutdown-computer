import React from 'react';

interface SectorProps {
    degree: number;  // 扇形角度 0~360
    color?: string;
    size?: number;
    start?: number;
}

const Sector: React.FC<SectorProps> = ({
    degree = 90,
    color = '#409eff',
    size = 56,
    start = 45,
}) => {
    return (
        <div
            className={`w-${size} h-${size}`}
            style={{
                borderRadius: '50%',
                background: `conic-gradient(${color} ${start}deg ${start + degree}deg, transparent ${start + degree}deg 360deg)`,
            }
            }
        />
    );
};

export default Sector;